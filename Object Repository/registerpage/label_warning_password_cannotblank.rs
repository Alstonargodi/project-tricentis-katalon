<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>label_warning_password_cannotblank</name>
   <tag></tag>
   <elementGuidId>4452c9ff-c182-4d0d-89b5-53ddd45039f5</elementGuidId>
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
      <webElementGuid>4e5b93fc-a3b6-4e9c-8c72-6d4e08512594</webElementGuid>
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



    
    


/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-2&quot;]/form[1]/div[@class=&quot;page registration-page&quot;]/div[@class=&quot;page-body&quot;]/div[@class=&quot;fieldset&quot;]/div[@class=&quot;form-fields&quot;]/div[@class=&quot;inputs&quot;]</value>
      <webElementGuid>c4150661-05f3-4dfd-8721-64fb07f64da8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>8741a7fb-4f69-481f-851a-3d0280b49a4c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>1207f9bb-4008-474c-b49b-26436d0078e0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
     &#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);&#xd;
        &#xd;
        &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
            &#xd;
        &#xd;
    &#xd;
    &#xd;
        &#xd;
    &#xd;
        &#xd;
            Register&#xd;
            Log in&#xd;
                            &#xd;
                &#xd;
                    Shopping cart&#xd;
                    (0)&#xd;
                &#xd;
            &#xd;
                    &#xd;
                &#xd;
                    Wishlist&#xd;
                    (0)&#xd;
                &#xd;
            &#xd;
        &#xd;
    &#xd;
        &#xd;
            $(document).ready(function () {&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
            });&#xd;
        &#xd;
&#xd;
&#xd;
        &#xd;
    &#xd;
        &#xd;
You have no items in your shopping cart.        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
    &#xd;
    &#xd;
    &#xd;
        $(document).ready(function() {&#xd;
            $(&quot;#small-searchterms&quot;).focus(function() {&#xd;
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {&#xd;
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;&#xd;
                }&#xd;
            });&#xd;
&#xd;
            $(&quot;#small-searchterms&quot;).blur(function() {&#xd;
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {&#xd;
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;&#xd;
                }&#xd;
            });&#xd;
        });&#xd;
&#xd;
        function check_small_search_form() {&#xd;
            var search_terms = $(&quot;#small-searchterms&quot;);&#xd;
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {&#xd;
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);&#xd;
                search_terms.focus();&#xd;
                return false;&#xd;
            }&#xd;
            return true;&#xd;
        }&#xd;
    &#xd;
        &#xd;
            &#xd;
                $(function() {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({&#xd;
                            delay: 500,&#xd;
                            minLength: 3,&#xd;
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,&#xd;
                            select: function(event, ui) {&#xd;
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);&#xd;
                                setLocation(ui.item.producturl);&#xd;
                                return false;&#xd;
                            }&#xd;
                        })&#xd;
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {&#xd;
                            var t = item.label;&#xd;
                            //html encode&#xd;
                            t = htmlEncode(t);&#xd;
                            return $(&quot;&lt;li>&lt;/li>&quot;)&#xd;
                                .data(&quot;item.autocomplete&quot;, item)&#xd;
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)&#xd;
                            .appendTo(ul);&#xd;
                    };&#xd;
                });&#xd;
            &#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
        &#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
        &#xd;
            &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        Books&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Computers&#xd;
        &#xd;
                &#xd;
                &#xd;
    &#xd;
        Desktops&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Notebooks&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Accessories&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Electronics&#xd;
        &#xd;
                &#xd;
                &#xd;
    &#xd;
        Camera, photo&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Cell phones&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Apparel &amp; Shoes&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Digital downloads&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Jewelry&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Gift Cards&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {&#xd;
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);&#xd;
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {&#xd;
            var width = $(this).innerWidth();&#xd;
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);&#xd;
        }&#xd;
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
    });&#xd;
&#xd;
    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {&#xd;
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);&#xd;
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
    });&#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
        &#xd;
            &#xd;
&#xd;
            Categories&#xd;
        &#xd;
    &#xd;
    &#xd;
        &#xd;
    &#xd;
        Books&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Computers&#xd;
        &#xd;
                &#xd;
                     &#xd;
                &#xd;
    &#xd;
        Desktops&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Notebooks&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Accessories&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Electronics&#xd;
        &#xd;
                &#xd;
                     &#xd;
                &#xd;
    &#xd;
        Camera, photo&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Cell phones&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Apparel &amp; Shoes&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Digital downloads&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Jewelry&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Gift Cards&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
        &#xd;
    &#xd;
    &#xd;
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {&#xd;
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);&#xd;
            },&#xd;
            function() {&#xd;
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);&#xd;
            }&#xd;
        );&#xd;
&#xd;
        $(function($) {&#xd;
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {&#xd;
                var parent = $(this).parent();&#xd;
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {&#xd;
                    $(&quot;.sublist:first&quot;, parent).hide(300);&#xd;
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                } else {&#xd;
                    $(&quot;.sublist:first&quot;, parent).show(300);&#xd;
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                }&#xd;
            });&#xd;
        });&#xd;
    &#xd;
&#xd;
        &#xd;
        &#xd;
        &#xd;
        &#xd;
            &#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
        &#xd;
            Categories&#xd;
        &#xd;
        &#xd;
            &#xd;
    &#xd;
        Books&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Computers&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Electronics&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Apparel &amp; Shoes&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Digital downloads&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Jewelry&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Gift Cards&#xd;
        &#xd;
&#xd;
    &#xd;
            &#xd;
        &#xd;
    &#xd;
    &#xd;
        &#xd;
            Manufacturers&#xd;
        &#xd;
        &#xd;
            &#xd;
                    Tricentis&#xd;
                    &#xd;
            &#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
        Newsletter&#xd;
    &#xd;
    &#xd;
        &#xd;
            Sign up for our newsletter:&#xd;
            &#xd;
            &#xd;
            &#xd;
            &#xd;
            &#xd;
                &#xd;
                Wait...&#xd;
            &#xd;
&#xd;
        &#xd;
        &#xd;
    &#xd;
    &#xd;
        $(document).ready(function () {&#xd;
            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-button&quot; , &quot;'&quot; , &quot;).click(function () {&#xd;
                &#xd;
                var email = $(&quot;#newsletter-email&quot;).val();&#xd;
                var subscribeProgress = $(&quot;#subscribe-loading-progress&quot;);&#xd;
                subscribeProgress.show();&#xd;
                $.ajax({&#xd;
                    cache: false,&#xd;
                    type: &quot;POST&quot;,&#xd;
                    url: &quot;/subscribenewsletter&quot;,&#xd;
                    data: { &quot;email&quot;: email },&#xd;
                    success: function (data) {&#xd;
                        subscribeProgress.hide();&#xd;
                        $(&quot;#newsletter-result-block&quot;).html(data.Result);&#xd;
                         if (data.Success) {&#xd;
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-block&quot; , &quot;'&quot; , &quot;).hide();&#xd;
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).show();&#xd;
                         }&#xd;
                         else {&#xd;
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).fadeIn(&quot;slow&quot;).delay(2000).fadeOut(&quot;slow&quot;);&#xd;
                         }&#xd;
                    },&#xd;
                    error:function (xhr, ajaxOptions, thrownError){&#xd;
                        alert(&quot; , &quot;'&quot; , &quot;Failed to subscribe.&quot; , &quot;'&quot; , &quot;);&#xd;
                        subscribeProgress.hide();&#xd;
                    }  &#xd;
                });                &#xd;
                return false;&#xd;
            });&#xd;
        });&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
            Register&#xd;
        &#xd;
        &#xd;
            &#xd;
                &#xd;
            &#xd;
            &#xd;
&#xd;
            &#xd;
                &#xd;
                    Your Personal Details&#xd;
                &#xd;
                &#xd;
                        &#xd;
                            Gender:&#xd;
                            &#xd;
                                &#xd;
                                Male&#xd;
                            &#xd;
                            &#xd;
                                &#xd;
                                Female&#xd;
                            &#xd;
                        &#xd;
                    &#xd;
                        First name:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                    &#xd;
                        Last name:&#xd;
                        &#xd;
                        *&#xd;
                        Last name is required.&#xd;
                    &#xd;
                    &#xd;
                        Email:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                &#xd;
            &#xd;
                                                            &#xd;
                &#xd;
                    Your Password&#xd;
                &#xd;
                &#xd;
                    &#xd;
                        Password:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                    &#xd;
                        Confirm password:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                &#xd;
            &#xd;
            &#xd;
                &#xd;
            &#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
&#xd;
&#xd;
        &#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
            Information&#xd;
            &#xd;
                    Sitemap&#xd;
                Shipping &amp; Returns&#xd;
                Privacy Notice&#xd;
                Conditions of Use&#xd;
                About us&#xd;
                Contact us&#xd;
            &#xd;
        &#xd;
        &#xd;
            Customer service&#xd;
            &#xd;
                Search &#xd;
                    News&#xd;
                                    Blog&#xd;
                                                    Recently viewed products&#xd;
                                    Compare products list&#xd;
                                    New products&#xd;
            &#xd;
        &#xd;
        &#xd;
            My account&#xd;
            &#xd;
                My account&#xd;
                    Orders&#xd;
                                    Addresses&#xd;
                                    Shopping cart&#xd;
                                    Wishlist&#xd;
            &#xd;
        &#xd;
        &#xd;
            Follow us&#xd;
            &#xd;
                    Facebook&#xd;
                                                    Twitter&#xd;
                                                    RSS&#xd;
                                                    YouTube&#xd;
                                                    Google+&#xd;
            &#xd;
        &#xd;
    &#xd;
    &#xd;
        Powered by nopCommerce&#xd;
        &#xd;
    &#xd;
    &#xd;
        Copyright © 2024 Tricentis Demo Web Shop. All rights reserved.&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
var _gaq = _gaq || [];&#xd;
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);&#xd;
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);&#xd;
&#xd;
(function() {&#xd;
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;&#xd;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;&#xd;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);&#xd;
})();&#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
&#xd;
&#xd;
/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-2&quot;]/form[1]/div[@class=&quot;page registration-page&quot;]/div[@class=&quot;page-body&quot;]/div[@class=&quot;fieldset&quot;]/div[@class=&quot;form-fields&quot;]/div[@class=&quot;inputs&quot;]&quot;) or . = concat(&quot;&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
     &#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);&#xd;
        &#xd;
        &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
            &#xd;
        &#xd;
    &#xd;
    &#xd;
        &#xd;
    &#xd;
        &#xd;
            Register&#xd;
            Log in&#xd;
                            &#xd;
                &#xd;
                    Shopping cart&#xd;
                    (0)&#xd;
                &#xd;
            &#xd;
                    &#xd;
                &#xd;
                    Wishlist&#xd;
                    (0)&#xd;
                &#xd;
            &#xd;
        &#xd;
    &#xd;
        &#xd;
            $(document).ready(function () {&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                });&#xd;
            });&#xd;
        &#xd;
&#xd;
&#xd;
        &#xd;
    &#xd;
        &#xd;
You have no items in your shopping cart.        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
    &#xd;
    &#xd;
    &#xd;
        $(document).ready(function() {&#xd;
            $(&quot;#small-searchterms&quot;).focus(function() {&#xd;
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {&#xd;
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;&#xd;
                }&#xd;
            });&#xd;
&#xd;
            $(&quot;#small-searchterms&quot;).blur(function() {&#xd;
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {&#xd;
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;&#xd;
                }&#xd;
            });&#xd;
        });&#xd;
&#xd;
        function check_small_search_form() {&#xd;
            var search_terms = $(&quot;#small-searchterms&quot;);&#xd;
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {&#xd;
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);&#xd;
                search_terms.focus();&#xd;
                return false;&#xd;
            }&#xd;
            return true;&#xd;
        }&#xd;
    &#xd;
        &#xd;
            &#xd;
                $(function() {&#xd;
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({&#xd;
                            delay: 500,&#xd;
                            minLength: 3,&#xd;
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,&#xd;
                            select: function(event, ui) {&#xd;
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);&#xd;
                                setLocation(ui.item.producturl);&#xd;
                                return false;&#xd;
                            }&#xd;
                        })&#xd;
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {&#xd;
                            var t = item.label;&#xd;
                            //html encode&#xd;
                            t = htmlEncode(t);&#xd;
                            return $(&quot;&lt;li>&lt;/li>&quot;)&#xd;
                                .data(&quot;item.autocomplete&quot;, item)&#xd;
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)&#xd;
                            .appendTo(ul);&#xd;
                    };&#xd;
                });&#xd;
            &#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
        &#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
        &#xd;
            &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        Books&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Computers&#xd;
        &#xd;
                &#xd;
                &#xd;
    &#xd;
        Desktops&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Notebooks&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Accessories&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Electronics&#xd;
        &#xd;
                &#xd;
                &#xd;
    &#xd;
        Camera, photo&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Cell phones&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Apparel &amp; Shoes&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Digital downloads&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Jewelry&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Gift Cards&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {&#xd;
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);&#xd;
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {&#xd;
            var width = $(this).innerWidth();&#xd;
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);&#xd;
        }&#xd;
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
    });&#xd;
&#xd;
    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {&#xd;
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);&#xd;
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
    });&#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
        &#xd;
            &#xd;
&#xd;
            Categories&#xd;
        &#xd;
    &#xd;
    &#xd;
        &#xd;
    &#xd;
        Books&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Computers&#xd;
        &#xd;
                &#xd;
                     &#xd;
                &#xd;
    &#xd;
        Desktops&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Notebooks&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Accessories&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Electronics&#xd;
        &#xd;
                &#xd;
                     &#xd;
                &#xd;
    &#xd;
        Camera, photo&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Cell phones&#xd;
        &#xd;
&#xd;
    &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Apparel &amp; Shoes&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Digital downloads&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Jewelry&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
    &#xd;
        Gift Cards&#xd;
        &#xd;
                &#xd;
&#xd;
    &#xd;
        &#xd;
    &#xd;
    &#xd;
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {&#xd;
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);&#xd;
            },&#xd;
            function() {&#xd;
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);&#xd;
            }&#xd;
        );&#xd;
&#xd;
        $(function($) {&#xd;
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {&#xd;
                var parent = $(this).parent();&#xd;
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {&#xd;
                    $(&quot;.sublist:first&quot;, parent).hide(300);&#xd;
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                } else {&#xd;
                    $(&quot;.sublist:first&quot;, parent).show(300);&#xd;
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);&#xd;
                }&#xd;
            });&#xd;
        });&#xd;
    &#xd;
&#xd;
        &#xd;
        &#xd;
        &#xd;
        &#xd;
            &#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
        &#xd;
            Categories&#xd;
        &#xd;
        &#xd;
            &#xd;
    &#xd;
        Books&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Computers&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Electronics&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Apparel &amp; Shoes&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Digital downloads&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Jewelry&#xd;
        &#xd;
&#xd;
    &#xd;
    &#xd;
        Gift Cards&#xd;
        &#xd;
&#xd;
    &#xd;
            &#xd;
        &#xd;
    &#xd;
    &#xd;
        &#xd;
            Manufacturers&#xd;
        &#xd;
        &#xd;
            &#xd;
                    Tricentis&#xd;
                    &#xd;
            &#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
        Newsletter&#xd;
    &#xd;
    &#xd;
        &#xd;
            Sign up for our newsletter:&#xd;
            &#xd;
            &#xd;
            &#xd;
            &#xd;
            &#xd;
                &#xd;
                Wait...&#xd;
            &#xd;
&#xd;
        &#xd;
        &#xd;
    &#xd;
    &#xd;
        $(document).ready(function () {&#xd;
            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-button&quot; , &quot;'&quot; , &quot;).click(function () {&#xd;
                &#xd;
                var email = $(&quot;#newsletter-email&quot;).val();&#xd;
                var subscribeProgress = $(&quot;#subscribe-loading-progress&quot;);&#xd;
                subscribeProgress.show();&#xd;
                $.ajax({&#xd;
                    cache: false,&#xd;
                    type: &quot;POST&quot;,&#xd;
                    url: &quot;/subscribenewsletter&quot;,&#xd;
                    data: { &quot;email&quot;: email },&#xd;
                    success: function (data) {&#xd;
                        subscribeProgress.hide();&#xd;
                        $(&quot;#newsletter-result-block&quot;).html(data.Result);&#xd;
                         if (data.Success) {&#xd;
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-subscribe-block&quot; , &quot;'&quot; , &quot;).hide();&#xd;
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).show();&#xd;
                         }&#xd;
                         else {&#xd;
                            $(&quot; , &quot;'&quot; , &quot;#newsletter-result-block&quot; , &quot;'&quot; , &quot;).fadeIn(&quot;slow&quot;).delay(2000).fadeOut(&quot;slow&quot;);&#xd;
                         }&#xd;
                    },&#xd;
                    error:function (xhr, ajaxOptions, thrownError){&#xd;
                        alert(&quot; , &quot;'&quot; , &quot;Failed to subscribe.&quot; , &quot;'&quot; , &quot;);&#xd;
                        subscribeProgress.hide();&#xd;
                    }  &#xd;
                });                &#xd;
                return false;&#xd;
            });&#xd;
        });&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
            Register&#xd;
        &#xd;
        &#xd;
            &#xd;
                &#xd;
            &#xd;
            &#xd;
&#xd;
            &#xd;
                &#xd;
                    Your Personal Details&#xd;
                &#xd;
                &#xd;
                        &#xd;
                            Gender:&#xd;
                            &#xd;
                                &#xd;
                                Male&#xd;
                            &#xd;
                            &#xd;
                                &#xd;
                                Female&#xd;
                            &#xd;
                        &#xd;
                    &#xd;
                        First name:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                    &#xd;
                        Last name:&#xd;
                        &#xd;
                        *&#xd;
                        Last name is required.&#xd;
                    &#xd;
                    &#xd;
                        Email:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                &#xd;
            &#xd;
                                                            &#xd;
                &#xd;
                    Your Password&#xd;
                &#xd;
                &#xd;
                    &#xd;
                        Password:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                    &#xd;
                        Confirm password:&#xd;
                        &#xd;
                        *&#xd;
                        &#xd;
                    &#xd;
                &#xd;
            &#xd;
            &#xd;
                &#xd;
            &#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
&#xd;
&#xd;
        &#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
            Information&#xd;
            &#xd;
                    Sitemap&#xd;
                Shipping &amp; Returns&#xd;
                Privacy Notice&#xd;
                Conditions of Use&#xd;
                About us&#xd;
                Contact us&#xd;
            &#xd;
        &#xd;
        &#xd;
            Customer service&#xd;
            &#xd;
                Search &#xd;
                    News&#xd;
                                    Blog&#xd;
                                                    Recently viewed products&#xd;
                                    Compare products list&#xd;
                                    New products&#xd;
            &#xd;
        &#xd;
        &#xd;
            My account&#xd;
            &#xd;
                My account&#xd;
                    Orders&#xd;
                                    Addresses&#xd;
                                    Shopping cart&#xd;
                                    Wishlist&#xd;
            &#xd;
        &#xd;
        &#xd;
            Follow us&#xd;
            &#xd;
                    Facebook&#xd;
                                                    Twitter&#xd;
                                                    RSS&#xd;
                                                    YouTube&#xd;
                                                    Google+&#xd;
            &#xd;
        &#xd;
    &#xd;
    &#xd;
        Powered by nopCommerce&#xd;
        &#xd;
    &#xd;
    &#xd;
        Copyright © 2024 Tricentis Demo Web Shop. All rights reserved.&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
&#xd;
&#xd;
var _gaq = _gaq || [];&#xd;
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);&#xd;
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);&#xd;
&#xd;
(function() {&#xd;
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;&#xd;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;&#xd;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);&#xd;
})();&#xd;
&#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
&#xd;
&#xd;
/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-2&quot;]/form[1]/div[@class=&quot;page registration-page&quot;]/div[@class=&quot;page-body&quot;]/div[@class=&quot;fieldset&quot;]/div[@class=&quot;form-fields&quot;]/div[@class=&quot;inputs&quot;]&quot;))]</value>
      <webElementGuid>3c59ee80-3ca5-4cbf-b1fa-34174ede56c3</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
