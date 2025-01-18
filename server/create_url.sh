curl -XPOST "localhost:7777/generate" \
-H 'Content-Type: application/json' \
-d '
{
	"code": "hehehe",
	"track_qr_scans": true,
	"destination": "https://cloudfare.com",
	"organization_id": 1,
	"active": true
}
'
	# "expiry_date": "2025-01-01 00:00:00",
	# "query_parameters": {
	# 	"utm_source": "tm",
	# 	"utm_medium": "okay"
	# }

