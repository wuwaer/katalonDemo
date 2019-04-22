<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getApiToken</name>
   <tag></tag>
   <elementGuidId>a931e741-3cbd-4461-959b-aebb80852323</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/cgi-bin/gettoken?corpid=ww5e2b49a3b4cd4f00&amp;corpsecret=qfvg8Wm3Kt5xDC4XeAU2ezChEwAowVWrLqi1QUhUpIs</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.wx_url</defaultValue>
      <description></description>
      <id>c27f2d21-cfce-4db3-9cd1-26c6ba904ae7</id>
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


WS.verifyElementPropertyValue(response, 'errmsg', 'ok')
def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(respone.getResponseText())

def token = jsonResponse.access_token


File file = new File(&quot;Data Files/testData/token.csv&quot;)
BufferedWriter out = new BufferedWriter(new FileWriter(file))
out.write(token)
out.flush()
out.close()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
