<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>cretdepartment</name>
   <tag></tag>
   <elementGuidId>d9281297-d0d3-45d0-ba23-6bdaa0a7f765</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;name\&quot;: \&quot;hhh2\&quot;,\n   \&quot;parentid\&quot;: 1,\n   \&quot;order\&quot;: 4,\n   \&quot;id\&quot;: ${num}\n}&quot;,
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
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://qyapi.weixin.qq.com/cgi-bin/department/create?access_token=${access_token}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.wx_url</defaultValue>
      <description></description>
      <id>764b24e5-f1e2-45f8-9fe4-4f14729f9e05</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>findTestData('testData/getapitoken').getValue(1, 1)</defaultValue>
      <description></description>
      <id>926f5cd4-06c8-4536-9c98-03bc8eb21b24</id>
      <masked>false</masked>
      <name>access_token</name>
   </variables>
   <variables>
      <defaultValue>findTestData('credepartmentdata/createdept').getValue(1, 1)</defaultValue>
      <description></description>
      <id>fe899710-3492-4373-9497-2a94d4975186</id>
      <masked>false</masked>
      <name>department</name>
   </variables>
   <variables>
      <defaultValue>findTestData('credepartmentdata/createdept').getValue(2, 1)</defaultValue>
      <description></description>
      <id>7cdc4f72-55d3-402c-a676-3805007af653</id>
      <masked>false</masked>
      <name>num</name>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
