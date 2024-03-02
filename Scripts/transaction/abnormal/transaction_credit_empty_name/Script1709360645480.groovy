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

WebUI.callTestCase(findTestCase('cart/normal/cart_insert'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('transactionpage/menu_shopping_cart'), 0)

WebUI.click(findTestObject('transactionpage/menu_shopping_cart'))

WebUI.verifyElementPresent(findTestObject('transactionpage/checkbox_term_of_service'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/button_checkout'), 0)

WebUI.click(findTestObject('Object Repository/transactionpage/checkbox_term_of_service'))

WebUI.click(findTestObject('transactionpage/button_checkout'))

WebUI.click(findTestObject('Object Repository/transactionpage/button_address'))

WebUI.click(findTestObject('transactionpage/radiobtn_payment_purchase'))

WebUI.click(findTestObject('Object Repository/transactionpage/button_next_step'))

WebUI.verifyElementPresent(findTestObject('transactionpage/div_payment_info'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/div_payment_method'), 0)

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_credit_card'), 'Visa', true)

WebUI.setText(findTestObject('transactionpage/textfield_card_holder_name'), '')

WebUI.setText(findTestObject('transactionpage/textfield_card_number'), '4001919257537193')

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_month'), '2026', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/transactionpage/option_year'), '9', true)

WebUI.setText(findTestObject('transactionpage/textfield_card_code'), '3525')

WebUI.click(findTestObject('transactionpage/button_next_confirm'))

WebUI.verifyElementPresent(findTestObject('transactionpage/div_confirm_order'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/div_billing_address_confirm'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/item_confirm'), 0)

WebUI.verifyElementPresent(findTestObject('transactionpage/button_confirm_finish'), 0)

WebUI.click(findTestObject('transactionpage/button_confirm_finish'), FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('transactionpage/text_confirm_success'), 0)

WebUI.closeBrowser()

