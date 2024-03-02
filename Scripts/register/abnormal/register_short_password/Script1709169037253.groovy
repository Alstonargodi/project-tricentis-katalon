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

WebUI.verifyElementPresent(findTestObject('registerpage/menu_register'), 0)

WebUI.click(findTestObject('Object Repository/registerpage/menu_register'))

WebUI.verifyElementPresent(findTestObject('registerpage/div_register_form'), 0)

WebUI.verifyElementPresent(findTestObject('registerpage/radiobtn_gender'), 0)

WebUI.click(findTestObject('Object Repository/registerpage/radiobtn_gender'))

WebUI.verifyElementPresent(findTestObject('registerpage/textfield_first_name'), 0)

WebUI.setText(findTestObject('Object Repository/registerpage/textfield_first_name'), 'wayan')

WebUI.verifyElementPresent(findTestObject('registerpage/textfield_last_name'), 0)

WebUI.setText(findTestObject('Object Repository/registerpage/textfield_last_name'), 'wayan')

WebUI.verifyElementPresent(findTestObject('registerpage/textfield_email'), 0)

WebUI.setText(findTestObject('Object Repository/registerpage/textfield_email'), 'wayanpnm4@mail.com')

WebUI.click(findTestObject('Object Repository/registerpage/div_confirm_password'))

WebUI.verifyElementPresent(findTestObject('registerpage/textfield_email'), 0)

WebUI.setEncryptedText(findTestObject('Object Repository/registerpage/textfield_password'), 'tzH6RvlfSTg=')

WebUI.verifyElementPresent(findTestObject('registerpage/textfield_confirm_password'), 0)

WebUI.setEncryptedText(findTestObject('Object Repository/registerpage/textfield_confirm_password'), 'tzH6RvlfSTg=')

WebUI.verifyElementPresent(findTestObject('Object Repository/registerpage/warning_password_length'), 
    0)

WebUI.closeBrowser()

