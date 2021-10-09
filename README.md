# Rust/Python Integration w/ Semi-Realworld Testcases

## Testing

Testing using the following Powershell 7.X command.
```powershell
1..1024 | ForEach-Object -Parallel { Invoke-WebRequest -Method Post -Body "{""data"": ""$(-join (Get-Random -Minimum 65 -Maximum 90 -Count (Get-Random -Minimum 128 -Maximum 2048) | % {[char]$_}))""}" -ContentType "application/json" 'http://localhost:8080/test' } -ThrottleLimit 32
```

## Misc Notes

* Rust type for a multithread shared queue of running Jobs (Futures)
  * `Arc<Mutex<VecDeque<Box<dyn Future<Output=Result<(), JoinError>> + Send>>>>`