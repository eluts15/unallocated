# unallocated


## Usage

Used to scan a Route53 HostedZone for "A" RecordSets that may be lingering around/not associated with an  
active EC2 instance in the account so that the entries can be removed.

```
./unallocated 
Fetching public IPs for associated instances.
Instance ID: i-014cf68da02a3ac7 IP Address: 32.23.23.123

Zone ID found, listing records in: "/hostedzone/SOME_HOSTED_ZONE_ID"

Existing Record found: []. Domain Name: example.com.
INFO: A record does not contain an IP example.com. --INFO

Existing Record found: ["32.23.23.123"]. Domain Name: test.example.com.
The record ["32.23.23.123"] appears to be valid.  --OK

Existing Record found: ["xx.xx.xx.xx"]. Domain Name: test1.example.com.
The record ["xx.xx.xx.xx"] for test1.example.com. appears to be unallocated. Consider deleting the record. --UNEXPECTED

Existing Record found: ["xx.xx.xx.xx"]. Domain Name: test2.example.com.
The record ["xx.xx.xx.xx"] for test2.example.com. appears to be unallocated. Consider deleting the record. --UNEXPECTED

Existing Record found: ["xx.xx.xx.xx"]. Domain Name: test3.example.com.
The record ["xx.xx.xx.xx"] for test3.example.com. appears to be unallocated. Consider deleting the record. --UNEXPECTED

Done.

```

## Setup 

TODO: Create an IAM User with the minumum permission set.

```
ADD EXAMPLE POLICY

```
Create an IAM user and attach the permission set (~/.aws/.credentials)

```

[your-profile]
aws_access_key_id=YOUR_ACCCES_KEY_ID
aws_secret_access_key=YOUR_SECRET_ACCESS_KEY
region=YOUR_REGION

```

Currently uses .env to map the PROFILE to credentials found in ~/.aws/credentials (not ideal, redundant)  
Create a `.env` file in the root directory.  

```
PROFILE=YOUR_PROFILE
```

## TODO

- Add tests  
- Test additional Hosted_Zones  


