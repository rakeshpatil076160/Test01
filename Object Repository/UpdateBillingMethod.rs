<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateBillingMethod</name>
   <tag></tag>
   <elementGuidId>65b23643-1d5d-4e9f-885d-ec8b3242ddd7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;PolicyNumber\&quot;: \&quot;019115077060004\&quot;,\n\t\&quot;ComCode\&quot;: \&quot;JH\&quot;,\n\t\&quot;EffectiveDate\&quot;: \&quot;2021-02-24T00:00:00-06:00\&quot;,\n\t\&quot;EventName\&quot;: null,\n\t\&quot;TokenNumber\&quot;: \&quot;cus_J0KrrGy5XmwIHU\&quot;,\n\t\&quot;RoutingNumber\&quot;: \&quot;021000021\&quot;,\n\t\&quot;AccountNumber\&quot;: \&quot;87865767676\&quot;,\n\t\&quot;BankName\&quot;: null,\n\t\&quot;AccountType\&quot;: \&quot;00000000-0000-0000-0000-000000000002\&quot;,\n\t\&quot;AccountHolderName\&quot;: null,\n\t\&quot;BillingType\&quot;: \&quot;onetime\&quot;,\n\t\&quot;BillingCategory\&quot;: \&quot;premium\&quot;,\n\t\&quot;BillingAmount\&quot;: \&quot;168.06\&quot;,\n\t\&quot;IsDefaultPayment\&quot;: \&quot;true\&quot;,\n\t\&quot;BillingMethod\&quot;: \&quot;CC\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>fc5d93fa-4eef-48b9-bf5d-90219dcdc91c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://se2fastwebqa.sbl.com/unifiedweb/api/updateBillingMethod</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
