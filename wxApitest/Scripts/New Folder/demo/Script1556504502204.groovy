import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.junit.After

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

def tokenresponse = WS.sendRequestAndVerify(findTestObject('getToken/getApiToken', [('url') : GlobalVariable.wx_url]))

def slurper = new groovy.json.JsonSlurper()
def result = slurper.parseText(tokenresponse.getResponseBodyContent())
def accessToken = result.access_token
println("... value" + accessToken)
GlobalVariable.token = accessToken
println("... tokenvalue"+ GlobalVariable.token)



def response = WS.sendRequest(findTestObject('deletedepart/delpartment', [('url') : GlobalVariable.wx_url, ('token') : findTestData(
                'gettoken/getapitoken').getValue(1, 1), ('partid') : partid]))

WS.verifyResponseStatusCode(response, 200, FailureHandling.STOP_ON_FAILURE)

def creresponse = WS.sendRequest(findTestObject('createrdepart/cretdepartment', [('url') : GlobalVariable.wx_url, ('access_token') : findTestData(
                'gettoken/getapitoken').getValue(1, 1), ('dep') : depment]))

WS.verifyResponseStatusCode(creresponse, 200)

println(creresponse)

WS.verifyElementPropertyValue(creresponse, 'errmsg', 'created')