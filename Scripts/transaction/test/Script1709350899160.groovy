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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demowebshop.tricentis.com/')

WebUI.click(findTestObject('Object Repository/transactionpage/a_Log in'))

WebUI.setText(findTestObject('Object Repository/transactionpage/input_Email_Email'), 'wayannpm@gmail.com')

WebUI.setEncryptedText(findTestObject('null'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.setText(findTestObject('null'), 'budi')

WebUI.setText(findTestObject('null'), 'budi@mail.com')

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_credit_card'), 'MasterCard', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_credit_card'), 'Discover', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_credit_card'), 'Amex', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_credit_card'), 'Visa', 
    true)

WebUI.click(findTestObject('null'))

WebUI.setText(findTestObject('null'), '43636')

WebUI.setText(findTestObject('null'), 'aadad')

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_month'), 
    '2026', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_year'), '2', true)

WebUI.setText(findTestObject('null'), '3525')

WebUI.closeBrowser()

