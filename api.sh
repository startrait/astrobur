BASE_URL="localhost:7777"

create_user() {
	curl -XPOST "${BASEURL}/user/create-user" \
	-H 'Content-Type: application/json' \
	-d '
		{
			"name" : "Shirish Maharjan",
			"email": "sirimhrzn@gmail.com",
			"password": "Shirish@123"
		}
	'
}

user_login() {
	curl -XPOST "${BASE_URL}/auth/user/login" \
	-H 'Content-Type: application/json' \
	-d '
		{
			"email": "sirimhrzn@gmail.com",
			"password": "Shirish@123"
		}
	'
}

create_url() {
	curl -XPOST "${BASE_URL}/api/url/generate" \
	-H 'Content-Type: application/json' \
	-d '
	{
		"code": "nepalipatro",
		"track_qr_scans": true,
		"destination": "https://nepalipatro.com.np",
		"organization_id": 1,
		"active": true
	}
	'
}

url_info() {
	local URL_CODE=$1
	local QUERY_PARAMS=${2:-""}
 	curl  "${BASE_URL}/api/url/$URL_CODE/info?$QUERY_PARAMS"  | jq
	
}

$1 $2 $3
