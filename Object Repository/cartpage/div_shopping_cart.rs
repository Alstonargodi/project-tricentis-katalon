<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_shopping_cart</name>
   <tag></tag>
   <elementGuidId>83f2c48a-f9fb-487a-b4d1-b798588d81be</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.page.shopping-cart-page</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
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
      <webElementGuid>ad1fe77b-6a5d-4ab7-8b24-59dbde83c55e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page shopping-cart-page</value>
      <webElementGuid>d1afd8e2-1679-49b5-bf86-2ee660a5458d</webElementGuid>
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
                                
                                    From: tono &lt;user_pnm@mail.com>For: wayan &lt;wayan_pnm@mail.com>
                                
                                                                                        
                                    Edit
                                
                                                    
                        
                            Price:
                            25.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            25.00
                        
                    
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            $25 Virtual Gift Card
                                
                                    From: wayan wayan &lt;wayan_pnm@mail.com>For: budi &lt;bud@mail.com>
                                
                                                                                        
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
                
                
                    50.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Not required
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            50.00
                    
                
            
        
    


                        
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
      <webElementGuid>74a3659a-f352-4969-b56b-edfc26436718</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-1&quot;]/div[@class=&quot;page shopping-cart-page&quot;]</value>
      <webElementGuid>67973a41-df83-4a74-bf0c-b977c5af27ea</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      <webElementGuid>0c8cc60b-c317-4f46-a2c9-ad00b9a7a020</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Jewelry'])[2]/following::div[7]</value>
      <webElementGuid>6fd91f47-ad51-4750-8f3d-533cab5a0214</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[4]/div/div</value>
      <webElementGuid>b905b6e5-d036-4cc1-ab32-83e651f327a4</webElementGuid>
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
                                
                                    From: tono &lt;user_pnm@mail.com>For: wayan &lt;wayan_pnm@mail.com>
                                
                                                                                        
                                    Edit
                                
                                                    
                        
                            Price:
                            25.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            25.00
                        
                    
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            $25 Virtual Gift Card
                                
                                    From: wayan wayan &lt;wayan_pnm@mail.com>For: budi &lt;bud@mail.com>
                                
                                                                                        
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
                
                
                    50.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Not required
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            50.00
                    
                
            
        
    


                        
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
                                
                                    From: tono &lt;user_pnm@mail.com>For: wayan &lt;wayan_pnm@mail.com>
                                
                                                                                        
                                    Edit
                                
                                                    
                        
                            Price:
                            25.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            25.00
                        
                    
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            $25 Virtual Gift Card
                                
                                    From: wayan wayan &lt;wayan_pnm@mail.com>For: budi &lt;bud@mail.com>
                                
                                                                                        
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
                
                
                    50.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Not required
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            50.00
                    
                
            
        
    


                        
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
      <webElementGuid>7153cdc0-8f30-406b-a8f4-eed2c8db4077</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
