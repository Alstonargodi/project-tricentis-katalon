<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_AjaxCart.init(false, .header-links .ca_ffb489_1</name>
   <tag></tag>
   <elementGuidId>c821eec6-e931-419c-a14a-eca92fdf5d27</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>ff9cafc7-f665-4dfa-894f-e394787fff7e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    







     





    
    
        
            AjaxCart.init(false, '.header-links .cart-qty', '.header-links .wishlist-qty', '#flyout-cart');
        
        


    
    
        
            
        
    
    
        
    
        
            Register
            Log in
                            
                
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
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $('li', '.top-menu').on('mouseenter', function () {
        $('a', $(this)).first().addClass('hover');
        if (!$(this).parent().hasClass('top-menu')) {
            var width = $(this).innerWidth();
            $('.sublist', $(this)).first().css('left', width + 15);
        }
        $('.sublist', $(this)).first().addClass('active');
        $('.top-menu-triangle', $(this)).addClass('active');
    });

    $('li', '.top-menu').on('mouseleave', function () {
        $('a', $(this)).first().removeClass('hover');
        $('.sublist', $(this)).first().removeClass('active');
        $('.top-menu-triangle', $(this)).removeClass('active');
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $('a', $('#mob-menu-button')).toggle(function() {
                $('.mob-top-menu').addClass('show');
            },
            function() {
                $('.mob-top-menu').removeClass('show');
            }
        );

        $(function($) {
            $('.mob-top-menu .expand').click(function() {
                var parent = $(this).parent();
                if (parent.hasClass('active')) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass('active');
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass('active');
                }
            });
        });
    

        
        
        
        
            
            
        
        
            



    
        
            Categories
        
        
            
    
        Books
        

    
    
        Computers
        

    
    
        Electronics
        

    
    
        Apparel &amp; Shoes
        

    
    
        Digital downloads
        

    
    
        Jewelry
        

    
    
        Gift Cards
        

    
            
        
    
    
        
            Manufacturers
        
        
            
                    Tricentis
                    
            
        
    

    
        Newsletter
    
    
        
            Sign up for our newsletter:
            
            
            
            
            
                
                Wait...
            

        
        
    
    
        $(document).ready(function () {
            $('#newsletter-subscribe-button').click(function () {
                
                var email = $(&quot;#newsletter-email&quot;).val();
                var subscribeProgress = $(&quot;#subscribe-loading-progress&quot;);
                subscribeProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;POST&quot;,
                    url: &quot;/subscribenewsletter&quot;,
                    data: { &quot;email&quot;: email },
                    success: function (data) {
                        subscribeProgress.hide();
                        $(&quot;#newsletter-result-block&quot;).html(data.Result);
                         if (data.Success) {
                            $('#newsletter-subscribe-block').hide();
                            $('#newsletter-result-block').show();
                         }
                         else {
                            $('#newsletter-result-block').fadeIn(&quot;slow&quot;).delay(2000).fadeOut(&quot;slow&quot;);
                         }
                    },
                    error:function (xhr, ajaxOptions, thrownError){
                        alert('Failed to subscribe.');
                        subscribeProgress.hide();
                    }  
                });                
                return false;
            });
        });
    



    
    
    
        
            Register
        
        
            
                
            
            

            
                
                    Your Personal Details
                
                
                        
                            Gender:
                            
                                
                                Male
                            
                            
                                
                                Female
                            
                        
                    
                        First name:
                        
                        *
                        
                    
                    
                        Last name:
                        
                        *
                        Last name is required.
                    
                    
                        Email:
                        
                        *
                        
                    
                
            
                                                            
                
                    Your Password
                
                
                    
                        Password:
                        
                        *
                        
                    
                    
                        Confirm password:
                        
                        *
                        
                    
                
            
            
                
            
        
    

    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2024 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push(['_setAccount', 'UA-6574346-11']);
_gaq.push(['_trackPageview']);

(function() {
    var ga = document.createElement('script'); ga.type = 'text/javascript'; ga.async = true;
    ga.src = ('https:' == document.location.protocol ? 'https://ssl' : 'http://www') + '.google-analytics.com/ga.js';
    var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-2&quot;]/form[1]/div[@class=&quot;page registration-page&quot;]/div[@class=&quot;page-body&quot;]/div[@class=&quot;fieldset&quot;]/div[@class=&quot;form-fields&quot;]/div[@class=&quot;inputs&quot;]/span[@class=&quot;field-validation-error&quot;]/span[1]</value>
      <webElementGuid>2bc06228-9a79-465f-960a-96ac34b5bab7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>f5445964-a4de-4c1f-b01c-0e765e7f40bc</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>ce857094-dff4-4a05-aec8-03e4794e7e1a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    







     





    
    
        
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);
        
        


    
    
        
            
        
    
    
        
    
        
            Register
            Log in
                            
                
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
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {
            var width = $(this).innerWidth();
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);
        }
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });

    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            },
            function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            }
        );

        $(function($) {
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {
                var parent = $(this).parent();
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    

        
        
        
        
            
            
        
        
            



    
        
            Categories
        
        
            
    
        Books
        

    
    
        Computers
        

    
    
        Electronics
        

    
    
        Apparel &amp; Shoes
        

    
    
        Digital downloads
        

    
    
        Jewelry
        

    
    
        Gift Cards
        

    
            
        
    
    
        
            Manufacturers
        
        
            
                    Tricentis
                    
            
        
    

    
        Newsletter
    
    
        
            Sign up for our newsletter:
            
            
            
            
            
                
                Wait...
            

        
        
    
    
        $(document).ready(function () {
            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-button&quot; , &quot;'&quot; , &quot;).click(function () {
                
                var email = $(&quot;#newsletter-email&quot;).val();
                var subscribeProgress = $(&quot;#subscribe-loading-progress&quot;);
                subscribeProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;POST&quot;,
                    url: &quot;/subscribenewsletter&quot;,
                    data: { &quot;email&quot;: email },
                    success: function (data) {
                        subscribeProgress.hide();
                        $(&quot;#newsletter-result-block&quot;).html(data.Result);
                         if (data.Success) {
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-block&quot; , &quot;'&quot; , &quot;).hide();
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).show();
                         }
                         else {
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).fadeIn(&quot;slow&quot;).delay(2000).fadeOut(&quot;slow&quot;);
                         }
                    },
                    error:function (xhr, ajaxOptions, thrownError){
                        alert(&quot; , &quot;'&quot; , &quot;Failed to subscribe.&quot; , &quot;'&quot; , &quot;);
                        subscribeProgress.hide();
                    }  
                });                
                return false;
            });
        });
    



    
    
    
        
            Register
        
        
            
                
            
            

            
                
                    Your Personal Details
                
                
                        
                            Gender:
                            
                                
                                Male
                            
                            
                                
                                Female
                            
                        
                    
                        First name:
                        
                        *
                        
                    
                    
                        Last name:
                        
                        *
                        Last name is required.
                    
                    
                        Email:
                        
                        *
                        
                    
                
            
                                                            
                
                    Your Password
                
                
                    
                        Password:
                        
                        *
                        
                    
                    
                        Confirm password:
                        
                        *
                        
                    
                
            
            
                
            
        
    

    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2024 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);

(function() {
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-2&quot;]/form[1]/div[@class=&quot;page registration-page&quot;]/div[@class=&quot;page-body&quot;]/div[@class=&quot;fieldset&quot;]/div[@class=&quot;form-fields&quot;]/div[@class=&quot;inputs&quot;]/span[@class=&quot;field-validation-error&quot;]/span[1]&quot;) or . = concat(&quot;
    







     





    
    
        
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);
        
        


    
    
        
            
        
    
    
        
    
        
            Register
            Log in
                            
                
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
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {
            var width = $(this).innerWidth();
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);
        }
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });

    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            },
            function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            }
        );

        $(function($) {
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {
                var parent = $(this).parent();
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    

        
        
        
        
            
            
        
        
            



    
        
            Categories
        
        
            
    
        Books
        

    
    
        Computers
        

    
    
        Electronics
        

    
    
        Apparel &amp; Shoes
        

    
    
        Digital downloads
        

    
    
        Jewelry
        

    
    
        Gift Cards
        

    
            
        
    
    
        
            Manufacturers
        
        
            
                    Tricentis
                    
            
        
    

    
        Newsletter
    
    
        
            Sign up for our newsletter:
            
            
            
            
            
                
                Wait...
            

        
        
    
    
        $(document).ready(function () {
            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-button&quot; , &quot;'&quot; , &quot;).click(function () {
                
                var email = $(&quot;#newsletter-email&quot;).val();
                var subscribeProgress = $(&quot;#subscribe-loading-progress&quot;);
                subscribeProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;POST&quot;,
                    url: &quot;/subscribenewsletter&quot;,
                    data: { &quot;email&quot;: email },
                    success: function (data) {
                        subscribeProgress.hide();
                        $(&quot;#newsletter-result-block&quot;).html(data.Result);
                         if (data.Success) {
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-block&quot; , &quot;'&quot; , &quot;).hide();
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).show();
                         }
                         else {
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).fadeIn(&quot;slow&quot;).delay(2000).fadeOut(&quot;slow&quot;);
                         }
                    },
                    error:function (xhr, ajaxOptions, thrownError){
                        alert(&quot; , &quot;'&quot; , &quot;Failed to subscribe.&quot; , &quot;'&quot; , &quot;);
                        subscribeProgress.hide();
                    }  
                });                
                return false;
            });
        });
    



    
    
    
        
            Register
        
        
            
                
            
            

            
                
                    Your Personal Details
                
                
                        
                            Gender:
                            
                                
                                Male
                            
                            
                                
                                Female
                            
                        
                    
                        First name:
                        
                        *
                        
                    
                    
                        Last name:
                        
                        *
                        Last name is required.
                    
                    
                        Email:
                        
                        *
                        
                    
                
            
                                                            
                
                    Your Password
                
                
                    
                        Password:
                        
                        *
                        
                    
                    
                        Confirm password:
                        
                        *
                        
                    
                
            
            
                
            
        
    

    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2024 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);

(function() {
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-2&quot;]/form[1]/div[@class=&quot;page registration-page&quot;]/div[@class=&quot;page-body&quot;]/div[@class=&quot;fieldset&quot;]/div[@class=&quot;form-fields&quot;]/div[@class=&quot;inputs&quot;]/span[@class=&quot;field-validation-error&quot;]/span[1]&quot;))]</value>
      <webElementGuid>2770393e-f4fc-4f81-9a83-149bbdb84119</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
