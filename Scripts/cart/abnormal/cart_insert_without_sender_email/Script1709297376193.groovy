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

WebUI.click(findTestObject('Object Repository/loginpage/menu_login'))

WebUI.verifyElementPresent(findTestObject('loginpage/div_login_page'), 0)

WebUI.verifyTextPresent('Welcome, Please Sign In!', false)

WebUI.verifyElementVisible(findTestObject('loginpage/textfield_email'))

WebUI.setText(findTestObject('Object Repository/loginpage/textfield_email'), username)

WebUI.verifyElementVisible(findTestObject('loginpage/textfield_password'))

WebUI.setText(findTestObject('loginpage/textfield_password'), password)

WebUI.verifyElementVisible(findTestObject('loginpage/checkbox_rememberme'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/loginpage/checkbox_rememberme'))

WebUI.verifyElementChecked(findTestObject('cartpage/checkbox_rememberme'), 1)

WebUI.verifyElementClickable(findTestObject('loginpage/button_login'))

WebUI.click(findTestObject('Object Repository/loginpage/button_login'))

WebUI.verifyElementPresent(findTestObject('cartpage/div_featured_product'), 0)

WebUI.click(findTestObject('cartpage/div_product_item'))

WebUI.verifyElementPresent(findTestObject('cartpage/textfield_recipient_name'), 0)

WebUI.verifyElementPresent(findTestObject('cartpage/textfield_recipient_email'), 0)

WebUI.verifyElementPresent(findTestObject('cartpage/textfield_sender_name'), 0)

WebUI.verifyElementPresent(findTestObject('cartpage/textfield_sender_email'), 0)

WebUI.verifyElementPresent(findTestObject('cartpage/button_quatity_cart'), 0)

WebUI.setText(findTestObject('Object Repository/cartpage/textfield_recipient_name'), recipient_name)

WebUI.setText(findTestObject('Object Repository/cartpage/textfield_recipient_email'), recipient_email)

WebUI.setText(findTestObject('cartpage/textfield_sender_name'), sender_name)

WebUI.setText(findTestObject('cartpage/textfield_sender_email'), '')

WebUI.click(findTestObject('Object Repository/cartpage/button_quatity_cart'))

WebUI.verifyElementVisible(findTestObject('cartpage/alert_sender_email'))

