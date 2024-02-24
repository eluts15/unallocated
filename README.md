# unallocated  


## Usage

Used to scan a Route53 HostedZone for "A" RecordSets that may be lingering around/not associated with an  
active EC2 instance in the account so that the entries can be removed if necessary.

```
./unallocated
     Running `target/debug/unallocated`                                                                                                                                                                            
Zone ID found, listing records in: "/hostedzone/HOSTED_ZONE_ID"                                                                                                                                     
                                                                                                                                                                                                     
Fetching public IPs for associated Linode Instances.                                                                                                                                                
Fetching public IPs for associated EC2 Instances.                                                                                                                                                                  
Address              | Domain Name                                        | Status                                                                                                                                 
No IP                | example.com.                                       | UNEXPECTED                                                                                                                             
xxx.xxx.xx.xx        | example2.com.                                      | UNEXPECTED                                                                                                                             
xxx.xxx.xx.xx        | somedomain.com.                                    | UNEXPECTED                                                                                                                          
xxx.xxx.xx.xx        | test.hello.com.                                    | OK                                                                                                                                     
xxx.xxx.xx.xx        | hello.com.                                         | OK                                                                                                                                     
xxx.xxx.xx.xx        | test.com.                                          | OK                                                                                                                                    
158 OK.
79 May require attention.
Iterated over 348 records.
Done

```
## Building for Release
```
Cargo build --release
```

## Setup 

TODO: Create an IAM User with the minumum permission set.

```
Requires ReadOnly Acess to Route53 and EC2

AmazonEC2ReadOnlyAccess
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": "ec2:Describe*",
            "Resource": "*"
        },
        {
            "Effect": "Allow",
            "Action": "elasticloadbalancing:Describe*",
            "Resource": "*"
        },
        {
            "Effect": "Allow",
            "Action": [
                "cloudwatch:ListMetrics",
                "cloudwatch:GetMetricStatistics",
                "cloudwatch:Describe*"
            ],
            "Resource": "*"
        },
        {
            "Effect": "Allow",
            "Action": "autoscaling:Describe*",
            "Resource": "*"
        }
    ]
}

AmazonRoute53ReadOnlyAccess
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "route53:Get*",
                "route53:List*",
                "route53:TestDNSAnswer"
            ],
            "Resource": [
                "*"
            ]
        }
    ]
}

LINODE/Akamai API KEY with ReadOnly Access

IPs
Linodes


```
Create an IAM user and attach the permission set above.

```

[your-profile]
aws_access_key_id=YOUR_ACCCES_KEY_ID
aws_secret_access_key=YOUR_SECRET_ACCESS_KEY
region=YOUR_REGION

```

Uses .env to set AWS Credentials.
Create a `.env` file in the root directory.  

## TODO/Bugs

- Add tests  
- Add support for multi-zone
    - Only checks the first zone in the list atm...
- Fix some things.
    - Doesn't check ec2 instances in other regions yet
- Configurable
    - 


