# coding: utf-8

"""
    lakeFS API

    lakeFS HTTP API

    The version of the OpenAPI document: 1.0.0
    Contact: services@treeverse.io
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest
import datetime

import lakefs_sdk
from lakefs_sdk.models.setup_state import SetupState  # noqa: E501
from lakefs_sdk.rest import ApiException

class TestSetupState(unittest.TestCase):
    """SetupState unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional):
        """Test SetupState
            include_option is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `SetupState`
        """
        model = lakefs_sdk.models.setup_state.SetupState()  # noqa: E501
        if include_optional :
            return SetupState(
                state = 'initialized', 
                comm_prefs_missing = True, 
                login_config = lakefs_sdk.models.login_config.LoginConfig(
                    rbac = 'none', 
                    username_ui_placeholder = '', 
                    password_ui_placeholder = '', 
                    login_url = '', 
                    login_failed_message = '', 
                    fallback_login_url = '', 
                    fallback_login_label = '', 
                    login_cookie_names = [
                        ''
                        ], 
                    logout_url = '', )
            )
        else :
            return SetupState(
        )
        """

    def testSetupState(self):
        """Test SetupState"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
