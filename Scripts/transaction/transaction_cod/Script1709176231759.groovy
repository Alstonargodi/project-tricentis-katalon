import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('login/login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('transactionpage/menu_shopping_cart'), 0)

WebUI.click(findTestObject('transactionpage/menu_shopping_cart'))

WebUI.verifyElementPresent(findTestObject('transactionpage/checkbox_term_of_service'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/button_checkout'), 0)

WebUI.click(findTestObject('Object Repository/transactionpage/checkbox_term_of_service'))

WebUI.click(findTestObject('transactionpage/button_checkout'))

WebUI.click(findTestObject('Object Repository/transactionpage/button_address'))

WebUI.click(findTestObject('Object Repository/transactionpage/radiobtn_payment_cod'))

WebUI.click(findTestObject('Object Repository/transactionpage/button_next_step'))

WebUI.verifyElementPresent(findTestObject('Object Repository/transactionpage/text_payment_info'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/div_payment_info'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/div_payment_method'), 0)

WebUI.click(findTestObject(null))

