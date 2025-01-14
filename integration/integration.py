import json
import os
import random

import unittest
from datetime import datetime
from http import HTTPStatus
from time import sleep

import docker
import requests
from selenium import webdriver
from selenium.webdriver.common.by import By


class EndToEndTests(unittest.TestCase):
    port = random.randint(10000, 65535)
    host = 'pantry'

    @classmethod
    def _cleanup(cls):
        client = docker.client.from_env()
        try:
            client.containers.get("pantry").remove(force=True)
        except:
            pass
        try:
            client.containers.get("selenium").remove(force=True)
        except:
            pass
        try:
            client.networks.get("test_net").remove()
        except:
            pass

    @classmethod
    def setUpClass(cls):
        cls._cleanup()
        client = docker.client.from_env()

        net = client.networks.create("test_net")
        addresses = [net.attrs['IPAM']['Config'][0]['Gateway'],
                     '127.0.0.1']
        client.images.build(path=f"{os.path.dirname(os.path.realpath(__file__))}/..", tag="pantry")
        client.containers.run("pantry", ports={"8080/tcp": cls.port}, detach=True, name="pantry", network="test_net")

        client.images.pull("selenium/standalone-firefox:latest")
        client.containers.run("selenium/standalone-firefox:latest",
                              shm_size='2g', detach=True, name="selenium", ports={"4444/tcp": 4444}, network="test_net")

        for i in range(60):
            try:
                addr = addresses[i%len(addresses)]
                requests.get(f"http://{addr}:4444", timeout=1)
                cls.dockerhost = addr
                break
            except requests.exceptions.ConnectionError:
                sleep(1)
                continue
        else:
            cls.fail(cls, f"Can't reach selenium box - http://{cls.dockerhost}:4444")

        options = webdriver.FirefoxOptions()
        cls.driver = webdriver.Remote(command_executor=f"http://{cls.dockerhost}:4444", options=options)

    @classmethod
    def tearDownClass(cls):
        cls._cleanup()

    def test_date_today(self):
        self.driver.get(f"http://{self.host}:8080/")
        self.driver.refresh()
        self.assertEqual("", self.driver.find_element(By.ID, "date").get_attribute('value'))

        self.driver.find_element(By.ID, "today").click()

        self.assertEqual(datetime.today().strftime('%Y-%m-%d'),
                         self.driver.find_element(By.ID, "date").get_attribute('value'))

    def test_end_to_end(self):
        resp1 = requests.put(f"http://{self.dockerhost}:{self.port}/api/item/1", json={"id": 1,
                                                                               "description": "First Item"})
        self.assertEqual(HTTPStatus.ACCEPTED, resp1.status_code, resp1.content)
        resp2 = requests.put(f"http://{self.dockerhost}:{self.port}/api/item/2", json={"id": 2,
                                                                               "description": "Second Item"})
        self.assertEqual(HTTPStatus.ACCEPTED, resp2.status_code, resp2.content)
        resp3 = requests.delete(f"http://{self.dockerhost}:{self.port}/api/item/3")
        self.assertIn(resp3.status_code, [HTTPStatus.OK, HTTPStatus.INTERNAL_SERVER_ERROR], resp3.content)

        self.driver.get(f"http://{self.host}:8080/#1")
        self.driver.refresh()
        self.assertEqual("1", self.driver.find_element(By.ID, "id").get_attribute('value'))
        self.assertEqual("First Item", self.driver.find_element(By.ID, "description").get_attribute('value'))

        self.driver.get(f"http://{self.host}:8080/#2")
        self.driver.refresh()
        self.assertEqual("2", self.driver.find_element(By.ID, "id").get_attribute('value'))
        self.assertEqual("Second Item", self.driver.find_element(By.ID, "description").get_attribute('value'))

        self.driver.get(f"http://{self.host}:8080/#3")
        self.driver.refresh()
        self.assertEqual("3", self.driver.find_element(By.ID, "id").get_attribute('value'))
        self.assertEqual("", self.driver.find_element(By.ID, "description").get_attribute('value'))

        self.driver.find_element(By.ID, "today").click()
        self.driver.find_element(By.ID, "description").send_keys("Third Item")
        self.driver.find_element(By.ID, "save").click()

        resp3 = requests.get(f"http://{self.dockerhost}:{self.port}/api/item/3")
        self.assertEqual(HTTPStatus.OK, resp3.status_code, resp3.content)
        item3 = json.loads(resp3.content)
        self.assertEqual("Third Item", item3["description"])

    def test_list(self):
        resp1 = requests.put(f"http://{self.dockerhost}:{self.port}/api/item/1", json={"id": 1,
                                                                                       "description": "First Item"})
        self.assertEqual(HTTPStatus.ACCEPTED, resp1.status_code, resp1.content)
        resp2 = requests.put(f"http://{self.dockerhost}:{self.port}/api/item/2", json={"id": 2,
                                                                                       "description": "Second Item"})
        self.assertEqual(HTTPStatus.ACCEPTED, resp2.status_code, resp2.content)
        resp3 = requests.delete(f"http://{self.dockerhost}:{self.port}/api/item/3")
        self.assertIn(resp3.status_code, [HTTPStatus.OK, HTTPStatus.INTERNAL_SERVER_ERROR], resp3.content)

        self.driver.get(f"http://{self.host}:8080/list/")

        self.assertEqual(1, len(self.driver.find_elements(by=By.XPATH, value="//*[contains(text(), 'First Item')]")))
        self.assertEqual(1, len(self.driver.find_elements(by=By.XPATH, value="//*[contains(text(), 'Second Item')]")))
        self.assertEqual(0, len(self.driver.find_elements(by=By.XPATH, value="//*[contains(text(), '3')]")))

        self.driver.find_elements(by=By.XPATH, value="//*[contains(text(), 'delete')]")[1].click()

        self.driver.get(f"http://{self.host}:8080/list/")

        self.assertEqual(0, len(self.driver.find_elements(by=By.XPATH, value="//*[contains(text(), 'Second Item')]")))

        self.driver.find_elements(by=By.XPATH, value="//*[contains(text(), '1')]")[0].click()

        self.assertEqual(self.driver.current_url, f"http://{self.host}:8080/#1")


if __name__ == '__main__':
    unittest.main()
