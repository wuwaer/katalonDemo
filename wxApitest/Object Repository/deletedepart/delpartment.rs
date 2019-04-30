<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>delpartment</name>
   <tag></tag>
   <elementGuidId>70e3ee98-6ed6-471c-afd8-59d1512af7b7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/cgi-bin/department/delete?access_token=${token}&amp;id=${partid}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.wx_url</defaultValue>
      <description></description>
      <id>f287f544-9f04-44da-81a8-4d1344a2f8d9</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>ea210f7f-2a15-484a-b308-4a9cb662e352</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>2917d030-acdc-4393-85d1-b3a9676e8786</id>
      <masked>false</masked>
      <name>partid</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'errmsg', 'deleted')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
