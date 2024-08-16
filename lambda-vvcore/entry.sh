#!/bin/bash
if [ -z "${AWS_LAMBDA_FUNCTION_NAME}" ]; then
  exec /usr/local/bin/aws-lambda-rie /usr/local/bin/lambda-vvcore
else
  exec /usr/local/bin/lambda-vvcore
fi