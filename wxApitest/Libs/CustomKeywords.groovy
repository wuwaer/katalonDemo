
/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import com.kms.katalon.core.testobject.TestObject

import java.lang.String


def static "gettoken.isexisttoken.verifyStatusCode"(
    	TestObject request	
     , 	int expectedStatusCode	) {
    (new gettoken.isexisttoken()).verifyStatusCode(
        	request
         , 	expectedStatusCode)
}

def static "gettoken.isexisttoken.addBasicAuthorizationProperty"(
    	TestObject request	
     , 	String username	
     , 	String password	) {
    (new gettoken.isexisttoken()).addBasicAuthorizationProperty(
        	request
         , 	username
         , 	password)
}
