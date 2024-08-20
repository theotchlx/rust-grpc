https://www.youtube.com/watch?v=kerKXChDmsE


Commands:

`cargo run --bin server`

`cargo run --bin client`


Requests:

`grpcurl -plaintext -d '{"a": 10, "b": 2}'  '[::1]:50051' calculator.Calculator.Add`

`grpcurl -plaintext -d '{"a": 10, "b": 2}'  '[::1]:50051' calculator.Calculator.Divide`


with auth:

`grpcurl -H "Authorization: Bearer some-secret-token" -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount`


without auth:

`grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount`

