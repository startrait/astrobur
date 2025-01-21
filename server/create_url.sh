curl -XPOST "localhost:7777/api/generate" \
-H 'Content-Type: application/json' \
-d '
{
	"code": "huha",
	"track_qr_scans": true,
	"destination": "https://cloudfare.com",
	"organization_id": 1,
	"active": true,
	"query_parameters": {
		"utm_source": "tm",
		"utm_medium": "okay"
	}
}
'
	# "expiry_date": "2025-01-01 00:00:00",


