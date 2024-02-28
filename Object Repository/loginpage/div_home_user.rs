<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_home_user</name>
   <tag></tag>
   <elementGuidId>31e29711-9165-4ede-9e16-95e9b5d77d2e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[4]/div/div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.header</value>
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
      <webElementGuid>d81e726a-59dc-406f-a8b0-89295cbc4705</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>header</value>
      <webElementGuid>2ebafdf7-729e-49bb-8ee1-81ac7b28ce93</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
        
            
        
    
    
        
    
        
            wayan_pnm@mail.com
            Log out
                            
                
                    Shopping cart
                    (0)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $('.header').on('mouseenter', '#topcartlink', function () {
                    $('#flyout-cart').addClass('active');
                });
                $('.header').on('mouseleave', '#topcartlink', function () {
                    $('#flyout-cart').removeClass('active');
                });
                $('.header').on('mouseenter', '#flyout-cart', function () {
                    $('#flyout-cart').addClass('active');
                });
                $('.header').on('mouseleave', '#flyout-cart', function () {
                    $('#flyout-cart').removeClass('active');
                });
            });
        


        
    
        
You have no items in your shopping cart.        
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == 'Search store') {
                    this.value = '';
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == '') {
                    this.value = 'Search store';
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert('Please enter some search keyword');
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $('#small-searchterms').autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: '/catalog/searchtermautocomplete',
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    
</value>
      <webElementGuid>f41d5263-c702-4f04-ae7d-de1ba1ea8e82</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;header&quot;]</value>
      <webElementGuid>d55f4855-afe4-4889-ba5f-7729e9a2889e</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[4]/div/div</value>
      <webElementGuid>6e84322b-b9e0-4836-9664-2e303590bd30</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
    
    
        
            
        
    
    
        
    
        
            wayan_pnm@mail.com
            Log out
                            
                
                    Shopping cart
                    (0)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
            });
        


        
    
        
You have no items in your shopping cart.        
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    
&quot;) or . = concat(&quot;
    
    
        
            
        
    
    
        
    
        
            wayan_pnm@mail.com
            Log out
                            
                
                    Shopping cart
                    (0)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
            });
        


        
    
        
You have no items in your shopping cart.        
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    
&quot;))]</value>
      <webElementGuid>1b0405f8-dee0-4ee7-b845-547156df39c8</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
