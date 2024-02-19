<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get details of all employees</description>
   <name>Get Employee List</name>
   <tag></tag>
   <elementGuidId>3f4b3c28-1e07-492c-9b80-c23b878269a3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://dummy.restapiexample.com/api/v1/employees</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>cba0852e-b9ad-415b-98fa-1c2cbca84204</id>
      <name>ValidateResponse</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\ahmdf\OneDrive\Desktop\JSONResponseSchema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'data[9].employee_name', &quot;Sonya Frost&quot;)

WS.verifyElementPropertyValue(response, 'data[9].employee_salary', 103600)

WS.verifyElementPropertyValue(response, 'data[9].employee_age', 23)

GlobalVariable.EMPLOYEE_NAME = WS.getElementPropertyValue(response, 'data[9].employee_name')

println 'Global Employee_Name is ' +GlobalVariable.EMPLOYEE_NAME</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
