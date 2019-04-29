import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequest(findTestObject('getToken/getApiToken', [('url') : GlobalVariable.wx_url]))

WS.sendRequest(findTestObject('deletedepart/delpartment', [('url') : GlobalVariable.wx_url, ('token') : findTestData('gettoken/getapitoken').getValue(
                1, 1), ('partid') : "${partid}"]))

WS.sendRequest(findTestObject('createrdepart/cretdepartment', [('url') : GlobalVariable.wx_url, ('access_token') : findTestData(
                'gettoken/getapitoken').getValue(1, 1), ('dep') : "${depment}"]))

