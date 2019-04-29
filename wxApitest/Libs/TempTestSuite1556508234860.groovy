import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.exception.StepFailedException
import com.kms.katalon.core.reporting.ReportUtil
import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.testdata.TestDataColumn
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import internal.GlobalVariable as GlobalVariable

Map<String, String> suiteProperties = new HashMap<String, String>();


suiteProperties.put('id', 'Test Suites/New Folder/New Test Suite')

suiteProperties.put('name', 'New Test Suite')

suiteProperties.put('description', '')
 

DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())



RunConfiguration.setExecutionSettingFile("/Users/dubl/katalonDemo/wxApitest/Reports/New Folder/New Test Suite/20190429_112354/execution.properties")

TestCaseMain.beforeStart()

TestCaseMain.startTestSuite('Test Suites/New Folder/New Test Suite', suiteProperties, [new TestCaseBinding('Test Cases/New Folder/demo - Iteration 1', 'Test Cases/New Folder/demo',  [ 'depment' : 'dep' ,  ]), new TestCaseBinding('Test Cases/New Folder/demo - Iteration 2', 'Test Cases/New Folder/demo',  [ 'depment' : '\u5E7F\u5DDE11' ,  ]), new TestCaseBinding('Test Cases/New Folder/demo - Iteration 3', 'Test Cases/New Folder/demo',  [ 'depment' : '\u676D\u5DDE11' ,  ]), new TestCaseBinding('Test Cases/New Folder/demo - Iteration 4', 'Test Cases/New Folder/demo',  [ 'depment' : '\u4E0A\u6D7711' ,  ]), new TestCaseBinding('Test Cases/New Folder/demo - Iteration 5', 'Test Cases/New Folder/demo',  [ 'depment' : '\u5317\u4EAC11' ,  ])])
