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
  &quot;text&quot;: &quot;{\n   \&quot;name\&quot;: \&quot;广州研发中心\&quot;,\n   \&quot;parentid\&quot;: 1,\n   \&quot;order\&quot;: 1,\n   \&quot;id\&quot;: 2\n}&quot;,
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
   <restUrl>${url}/cgi-bin/department/create?access_token=a1p4cx9t6iaaGc1vl4KO2GX7lgTc6qp9hTMIRLN8TeEVdtSJInQgnEPF0XVU7llEo3Df4U3fFW8NRBMYkHQZNE5Hojx3gyQaJOqiYS_s5DShI1ed8FSDhQwwAN5xApiqaaQgUnIkUO4ioiZSEKbiBnINisJpPqT1jUSJXKgAXbxmuqV8IgMEahyVLMoeGv1vSvAOQmXBnVotsiTqALVC_Q</restUrl>
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
