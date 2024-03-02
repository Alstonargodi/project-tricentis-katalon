<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_billing_address</name>
   <tag></tag>
   <elementGuidId>6faa271b-8b01-4edf-bf7f-9800696b2551</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='billing-new-address-form']/div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.enter-address</value>
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
      <webElementGuid>8e6c9198-2076-461f-94f9-3b0ba5241935</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>enter-address</value>
      <webElementGuid>e743aa59-f119-427c-ac4d-35cac204caca</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

    
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
                
            
        


            
        </value>
      <webElementGuid>76625954-42ea-4563-807d-a8887fcd449d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;billing-new-address-form&quot;)/div[@class=&quot;enter-address&quot;]</value>
      <webElementGuid>8799b4c8-17b0-4303-8765-463690fa7c3e</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='billing-new-address-form']/div</value>
      <webElementGuid>1b5b864f-d221-4267-91b7-266d776d9d05</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Billing address'])[1]/following::div[5]</value>
      <webElementGuid>78fc96d2-a363-456f-bd96-ca80b89cf27e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Checkout'])[1]/following::div[7]</value>
      <webElementGuid>2ee2c904-6368-4212-bae1-b54cfd311a5b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div/div/div/div</value>
      <webElementGuid>10d756d8-0dd9-4f54-85c8-475fcaa8d1fa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
            

    
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
                
            
        


            
        &quot;) or . = concat(&quot;
            

    
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
                
            
        


            
        &quot;))]</value>
      <webElementGuid>2f13acfa-db03-4465-80b5-e4ca56506deb</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
