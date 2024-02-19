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

response = WS.sendRequest(findTestObject('API/SOAP/Applying Parameterization and Variables to API Tests/CountryInfoServiceSoapBinding/CountryISOCode', 
        [('countryName') :countryName ]))

WS.verifyElementText(response, 'CountryISOCodeResponse.CountryISOCodeResult', countryCode)

response1 = WS.sendRequest(findTestObject('API/SOAP/Applying Parameterization and Variables to API Tests/CountryInfoServiceSoapBinding/CapitalCity', 
        [('countryCode') : countryCode]))

WS.verifyElementText(response1, 'CapitalCityResponse.CapitalCityResult', capitalCity)

response2 = WS.sendRequest(findTestObject('API/SOAP/Applying Parameterization and Variables to API Tests/CountryInfoServiceSoapBinding/CountryCurrency', 
        [('countryCode') : countryCode]))

WS.verifyElementText(response2, 'CountryCurrencyResponse.CountryCurrencyResult.sISOCode', currencyName)

