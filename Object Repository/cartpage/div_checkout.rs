<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_checkout</name>
   <tag></tag>
   <elementGuidId>cc9b6af8-ef2c-44ec-8cbd-bc14503fbf68</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.page.checkout-page</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>f47386e2-a38f-4cf0-b4ad-52fa17f6626a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page checkout-page</value>
      <webElementGuid>a19290cb-011b-4167-ba90-03c89d1b396b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        Checkout
    
    
        
            
                
                    1
                    Billing address
                
                
                    
                    
                        
    
        
            Select a billing address from your address book or enter a new address.
            
                
                        wayan wayan, sydney city, sydney 21415, Australia
                    New Address
                
            
        
    
    
        
            

    
        $(function () {
            $(&quot;#BillingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#BillingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html('');
                        $.each(data, function (id, option) {
                            ddlStates.append($('&lt;option>&lt;/option>').val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert('Failed to retrieve states.');
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D'Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People's Republic of
Kuwait
Kyrgyzstan
Lao People's Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


            
        
    
    


                        
                    
                    
                    
                        Billing.init('#co-billing-form', 'https://demowebshop.tricentis.com/checkout/OpcSaveBilling/', false);
                        if ($(&quot;#billing-address-select&quot;).length > 0) {
                            Billing.newAddress(!$('#billing-address-select').val());
                        }
                    
                    
                        
                        Loading next step...
                    
                
            
            
                
                    2
                    Payment method
                
                
                    
                    
                         Payment is not required
                    
                    
                    
                        PaymentMethod.init('#co-payment-method-form', 'https://demowebshop.tricentis.com/checkout/OpcSavePaymentMethod/');
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    3
                    Payment information
                
                
                    
                    
                         Payment is not required
                    
                    
                    
                        PaymentInfo.init('#co-payment-info-form', 'https://demowebshop.tricentis.com/checkout/OpcSavePaymentInfo/');
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    4
                    Confirm order
                
                
                    
                        
                    
                    
                        ConfirmOrder.init('https://demowebshop.tricentis.com/checkout/OpcConfirmOrder/', 'https://demowebshop.tricentis.com/checkout/completed/');
                    
                    
                        
                            « Back
                        
                        Submitting order information...
                    
                
            
        
    
    
        Accordion.init('checkout-steps', '.step-title', true);
        Accordion.openSection('#opc-billing');
        Checkout.init('https://demowebshop.tricentis.com/cart/');
        if (Billing.disableBillingAddressCheckoutStep)
        {
            Accordion.hideSection('#opc-billing');
            Billing.save();
        }
    
</value>
      <webElementGuid>5db5d1ae-5b60-45ce-b446-609755e8cef2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-1&quot;]/div[@class=&quot;page checkout-page&quot;]</value>
      <webElementGuid>9242106d-a695-45c1-a1c6-6e43e3ce1358</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      <webElementGuid>2c89fb04-661b-463c-934b-f46e90a2cd03</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Jewelry'])[2]/following::div[7]</value>
      <webElementGuid>3be6766c-65cd-4b08-87d6-63c6e859a69e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[4]/div/div</value>
      <webElementGuid>e7907661-6efa-42b7-b457-b37a968aa7aa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
    
        Checkout
    
    
        
            
                
                    1
                    Billing address
                
                
                    
                    
                        
    
        
            Select a billing address from your address book or enter a new address.
            
                
                        wayan wayan, sydney city, sydney 21415, Australia
                    New Address
                
            
        
    
    
        
            

    
        $(function () {
            $(&quot;#BillingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#BillingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $.each(data, function (id, option) {
                            ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


            
        
    
    


                        
                    
                    
                    
                        Billing.init(&quot; , &quot;'&quot; , &quot;#co-billing-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveBilling/&quot; , &quot;'&quot; , &quot;, false);
                        if ($(&quot;#billing-address-select&quot;).length > 0) {
                            Billing.newAddress(!$(&quot; , &quot;'&quot; , &quot;#billing-address-select&quot; , &quot;'&quot; , &quot;).val());
                        }
                    
                    
                        
                        Loading next step...
                    
                
            
            
                
                    2
                    Payment method
                
                
                    
                    
                         Payment is not required
                    
                    
                    
                        PaymentMethod.init(&quot; , &quot;'&quot; , &quot;#co-payment-method-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentMethod/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    3
                    Payment information
                
                
                    
                    
                         Payment is not required
                    
                    
                    
                        PaymentInfo.init(&quot; , &quot;'&quot; , &quot;#co-payment-info-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentInfo/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    4
                    Confirm order
                
                
                    
                        
                    
                    
                        ConfirmOrder.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcConfirmOrder/&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/completed/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Submitting order information...
                    
                
            
        
    
    
        Accordion.init(&quot; , &quot;'&quot; , &quot;checkout-steps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.step-title&quot; , &quot;'&quot; , &quot;, true);
        Accordion.openSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
        Checkout.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/cart/&quot; , &quot;'&quot; , &quot;);
        if (Billing.disableBillingAddressCheckoutStep)
        {
            Accordion.hideSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
            Billing.save();
        }
    
&quot;) or . = concat(&quot;
    
        Checkout
    
    
        
            
                
                    1
                    Billing address
                
                
                    
                    
                        
    
        
            Select a billing address from your address book or enter a new address.
            
                
                        wayan wayan, sydney city, sydney 21415, Australia
                    New Address
                
            
        
    
    
        
            

    
        $(function () {
            $(&quot;#BillingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#BillingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $.each(data, function (id, option) {
                            ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


            
        
    
    


                        
                    
                    
                    
                        Billing.init(&quot; , &quot;'&quot; , &quot;#co-billing-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveBilling/&quot; , &quot;'&quot; , &quot;, false);
                        if ($(&quot;#billing-address-select&quot;).length > 0) {
                            Billing.newAddress(!$(&quot; , &quot;'&quot; , &quot;#billing-address-select&quot; , &quot;'&quot; , &quot;).val());
                        }
                    
                    
                        
                        Loading next step...
                    
                
            
            
                
                    2
                    Payment method
                
                
                    
                    
                         Payment is not required
                    
                    
                    
                        PaymentMethod.init(&quot; , &quot;'&quot; , &quot;#co-payment-method-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentMethod/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    3
                    Payment information
                
                
                    
                    
                         Payment is not required
                    
                    
                    
                        PaymentInfo.init(&quot; , &quot;'&quot; , &quot;#co-payment-info-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentInfo/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    4
                    Confirm order
                
                
                    
                        
                    
                    
                        ConfirmOrder.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcConfirmOrder/&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/completed/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Submitting order information...
                    
                
            
        
    
    
        Accordion.init(&quot; , &quot;'&quot; , &quot;checkout-steps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.step-title&quot; , &quot;'&quot; , &quot;, true);
        Accordion.openSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
        Checkout.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/cart/&quot; , &quot;'&quot; , &quot;);
        if (Billing.disableBillingAddressCheckoutStep)
        {
            Accordion.hideSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
            Billing.save();
        }
    
&quot;))]</value>
      <webElementGuid>cf3c9c25-8269-4c8c-acc3-b9a8f0677136</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
