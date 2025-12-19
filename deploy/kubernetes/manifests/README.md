INSTALLATION
---

A secret `pushover-credentials` containing `PUSHOVER_TOKEN` and `PUSHOVER_USER_KEY`.

```yaml
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: pushover-credentials
type: Opaque
data:
  PUSHOVER_TOKEN: asdf
  PUSHOVER_USER_KEY: asdf
```
```
