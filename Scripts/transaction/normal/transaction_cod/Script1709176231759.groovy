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

WebUI.comment('Login page')

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

WebUI.comment('Insert a item to cart')

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

WebUI.setText(findTestObject('cartpage/textfield_sender_email'), sender_email)

WebUI.setText(findTestObject('cartpage/textfield_message'), sender_message)

WebUI.click(findTestObject('Object Repository/cartpage/button_quatity_cart'))

WebUI.verifyElementVisible(findTestObject('cartpage/label_add_success'))

WebUI.comment('transacition page')

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

WebUI.click(findTestObject('transactionpage/button_next_confirm'))

WebUI.verifyElementPresent(findTestObject('transactionpage/div_confirm_order'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/div_billing_address_confirm'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/item_confirm'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/button_confirm_finish'), 0)

WebUI.click(findTestObject('transactionpage/button_confirm_finish'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('transactionpage/text_confirm_success'), 0)

WebUI.closeBrowser()

