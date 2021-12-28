#!/usr/bin/env python3

# This is a variation of send-done-email.py created for provisioning.  If this
# doesn't end up getting specialized to perform a grep, this could potentially
# be unified.

from __future__ import absolute_import
import sys
import boto3
import os
import subprocess

client = boto3.client('ses')
subj_prefix = sys.argv[1]
dest_email = sys.argv[2]
what_happened = sys.argv[3]

response = client.send_email(
    Source='daemon@searchfox.org',
    Destination={
        'ToAddresses': [
            dest_email,
        ]
    },
    Message={
        'Subject': {
            'Data': f'{subj_prefix} Searchfox provisioning {what_happened}',
        },
        'Body': {
            'Text': {
                'Data': f'Searchfox provisioning {what_happened}!',
            },
        }
    }
)
