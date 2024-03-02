<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>page_shopping_cart</name>
   <tag></tag>
   <elementGuidId>01331348-eb8f-4cfb-bee6-4dbd3fd28f03</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.page.shopping-cart-page</value>
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
      <webElementGuid>4707a1fd-a159-4959-8e38-ee4c17253a8b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page shopping-cart-page</value>
      <webElementGuid>48b9cb82-754c-4171-a473-3a7afb32017b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            $25 Virtual Gift Card
                                
                                    From: wayan &lt;wayanpnm@gmail.com>For: budi &lt;budi@mail.com>
                                
                                                                                        
                                    Edit
                                
                                                    
                        
                            Price:
                            25.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            25.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    25.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Not required
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            25.00
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $('#checkout').click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($('#termsofservice').length > 0) {
                                            //terms of service element exists
                                            if (!$('#termsofservice').is(':checked')) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    
</value>
      <webElementGuid>4dd03620-8678-4a98-b891-6d60ff925613</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-1&quot;]/div[@class=&quot;page shopping-cart-page&quot;]</value>
      <webElementGuid>9f022578-b3ac-4dbc-9c77-9d8483b9e125</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      <webElementGuid>d654e1d5-d585-4e54-aa1b-6fc0bda9e3d5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Jewelry'])[2]/following::div[7]</value>
      <webElementGuid>b170c702-9d3b-46b3-8673-2f41aae4d597</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[4]/div/div</value>
      <webElementGuid>6e337267-a787-4a68-ac99-b81941b2dfaa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            $25 Virtual Gift Card
                                
                                    From: wayan &lt;wayanpnm@gmail.com>For: budi &lt;budi@mail.com>
                                
                                                                                        
                                    Edit
                                
                                                    
                        
                            Price:
                            25.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            25.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    25.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Not required
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            25.00
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $(&quot; , &quot;'&quot; , &quot;#checkout&quot; , &quot;'&quot; , &quot;).click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).length > 0) {
                                            //terms of service element exists
                                            if (!$(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    
&quot;) or . = concat(&quot;
    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            $25 Virtual Gift Card
                                
                                    From: wayan &lt;wayanpnm@gmail.com>For: budi &lt;budi@mail.com>
                                
                                                                                        
                                    Edit
                                
                                                    
                        
                            Price:
                            25.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            25.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    25.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Not required
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            25.00
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $(&quot; , &quot;'&quot; , &quot;#checkout&quot; , &quot;'&quot; , &quot;).click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).length > 0) {
                                            //terms of service element exists
                                            if (!$(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    
&quot;))]</value>
      <webElementGuid>5e4bef40-955a-4101-89a9-c32770baa770</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
