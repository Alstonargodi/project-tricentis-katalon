<?xml version="1.0" encoding="UTF-8"?>
<TestSuiteEntity>
   <description></description>
   <name>register_case</name>
   <tag></tag>
   <isRerun>false</isRerun>
   <mailRecipient></mailRecipient>
   <numberOfRerun>3</numberOfRerun>
   <pageLoadTimeout>30</pageLoadTimeout>
   <pageLoadTimeoutDefault>true</pageLoadTimeoutDefault>
   <rerunFailedTestCasesOnly>false</rerunFailedTestCasesOnly>
   <rerunImmediately>true</rerunImmediately>
   <testSuiteGuid>6bca1b95-bd6c-4cc7-a961-15be3a8ffb39</testSuiteGuid>
   <testCaseLink>
      <guid>94687ff1-63a0-4453-9298-bbaee0298d99</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/register/normal/register_normal</testCaseId>
      <testDataLink>
         <combinationType>ONE</combinationType>
         <id>75147e31-d0ea-4097-ba76-f78fa17047c3</id>
         <iterationEntity>
            <iterationType>ALL</iterationType>
            <value></value>
         </iterationEntity>
         <testDataId>Data Files/register/register_data_normal</testDataId>
      </testDataLink>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
      <variableLink>
         <testDataLinkId>75147e31-d0ea-4097-ba76-f78fa17047c3</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>first_name</value>
         <variableId>28786cf7-6583-4487-94d4-ed30badfd4e2</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>75147e31-d0ea-4097-ba76-f78fa17047c3</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>last_name</value>
         <variableId>5bdb3120-f6ec-47d4-96c4-39755a2370dd</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>75147e31-d0ea-4097-ba76-f78fa17047c3</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>password</value>
         <variableId>0c255084-c87a-416c-84f8-d3aa2f1330b5</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>75147e31-d0ea-4097-ba76-f78fa17047c3</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>confirm_password</value>
         <variableId>26c3833f-a586-42e6-b4b3-2aa5766588fe</variableId>
      </variableLink>
   </testCaseLink>
   <testCaseLink>
      <guid>1390d78b-6516-42b1-bbd0-971931239460</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/register/abnormal/register_short_password</testCaseId>
      <testDataLink>
         <combinationType>ONE</combinationType>
         <id>06fb8da5-a591-4a6e-82b9-ca7301bf95f0</id>
         <iterationEntity>
            <iterationType>ALL</iterationType>
            <value></value>
         </iterationEntity>
         <testDataId>Data Files/register/register_data_short</testDataId>
      </testDataLink>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
      <variableLink>
         <testDataLinkId>06fb8da5-a591-4a6e-82b9-ca7301bf95f0</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>first_name</value>
         <variableId>5f5d773c-1aae-488b-91de-337d72e2306d</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>06fb8da5-a591-4a6e-82b9-ca7301bf95f0</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>last_name</value>
         <variableId>d2caad78-dd0a-425f-9eeb-1157a87357f8</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>06fb8da5-a591-4a6e-82b9-ca7301bf95f0</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>password</value>
         <variableId>4417670d-9b84-4e9a-94ef-23e0136d3666</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>06fb8da5-a591-4a6e-82b9-ca7301bf95f0</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>confirm_password</value>
         <variableId>0be27d02-1580-4616-81c5-b8f31ff475fc</variableId>
      </variableLink>
   </testCaseLink>
   <testCaseLink>
      <guid>7239d901-0f54-456d-885f-4d2efd37515f</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/register/abnormal/register_wrong_confirm_password</testCaseId>
      <testDataLink>
         <combinationType>ONE</combinationType>
         <id>580777a4-27c0-43a7-9005-76ec8beac4f9</id>
         <iterationEntity>
            <iterationType>ALL</iterationType>
            <value></value>
         </iterationEntity>
         <testDataId>Data Files/register/register_data_wrongconfirm</testDataId>
      </testDataLink>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
      <variableLink>
         <testDataLinkId>580777a4-27c0-43a7-9005-76ec8beac4f9</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>first_name</value>
         <variableId>056f95ee-1340-4f99-be32-6f661a1d8a0e</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>580777a4-27c0-43a7-9005-76ec8beac4f9</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>last_name</value>
         <variableId>de604c54-9bbf-485e-8cda-85ea7ecb0e35</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>580777a4-27c0-43a7-9005-76ec8beac4f9</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>password</value>
         <variableId>4102565a-05a6-4ebe-be2e-bd5936a3f758</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>580777a4-27c0-43a7-9005-76ec8beac4f9</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>confirm_password</value>
         <variableId>76a293cd-d24f-4159-a960-a7d9c4a6e799</variableId>
      </variableLink>
   </testCaseLink>
</TestSuiteEntity>
