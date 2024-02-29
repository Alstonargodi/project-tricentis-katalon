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

WebUI.setText(findTestObject('Object Repository/transactionpage/input_Email_Email'), 'wayan_pnm@mail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/transactionpage/input_Password_Password'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('Object Repository/transactionpage/input_Password_RememberMe'))

WebUI.click(findTestObject('Object Repository/transactionpage/input_Forgot password_button-1 login-button'))

WebUI.click(findTestObject('Object Repository/transactionpage/span_Shopping cart'))

WebUI.click(findTestObject('Object Repository/transactionpage/input_I agree with the terms of service and_f529a0'))

WebUI.click(findTestObject('Object Repository/transactionpage/button_Checkout'))

WebUI.click(findTestObject('Object Repository/transactionpage/input_Fax number_button-1 new-address-next-_2f210c'))

WebUI.click(findTestObject('Object Repository/transactionpage/input_Payment method_paymentmethod'))

WebUI.click(findTestObject('Object Repository/transactionpage/input__button-1 payment-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/transactionpage/p_You will pay by COD'))

WebUI.click(findTestObject('Object Repository/transactionpage/input__button-1 payment-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/transactionpage/div_Sub-Total                              _27cc70'))

WebUI.click(findTestObject('Object Repository/transactionpage/input__button-1 payment-method-next-step-button'))

WebUI.click(findTestObject('Object Repository/transactionpage/strong_Your order has been successfully processed'))

WebUI.click(findTestObject('Object Repository/transactionpage/input__button-1 payment-method-next-step-button'))

WebUI.closeBrowser()

