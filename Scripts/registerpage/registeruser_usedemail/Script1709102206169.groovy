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

WebUI.click(findTestObject('Object Repository/registerpage/menuitem_register'))

WebUI.click(findTestObject('Object Repository/registerpage/input_Gender_Gender'))

WebUI.setText(findTestObject('Object Repository/registerpage/input_First name_FirstName'), 'wayan')

WebUI.setText(findTestObject('Object Repository/registerpage/input_Last name_LastName'), 'wayan')

WebUI.setText(findTestObject('Object Repository/registerpage/input_Email_Email'), 'wayanpnm@gmail.com')

WebUI.click(findTestObject('Object Repository/registerpage/div_Your Password                          _3011b2'))

WebUI.setEncryptedText(findTestObject('Object Repository/registerpage/input_Password_Password'), 'aeHFOx8jV/A=')

WebUI.setEncryptedText(findTestObject('Object Repository/registerpage/input_Confirm password_ConfirmPassword'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('Object Repository/registerpage/input__register-button'))

WebUI.closeBrowser()

