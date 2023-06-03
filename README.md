# rust编写的基于actix web的紫微斗数排盘server

## 单元测试

* 下载瑞士星历表，并编译
```bash
mkdir /tmp/swe
cd /tmp/swe
wget https://www.astro.com/ftp/swisseph/swe_unix_src_2.10.03.tar.gz
tar xvzf swe_unix_src_2.10.03.tar.gz
cd src
make libswe.so
```

* 单元测试
```bash
RUSTFLAGS=-L/tmp/swe/src LD_LIBRARY_PATH=/tmp/swe/src cargo test
```


## 请求api
http://localhost:8080/swagger-ui/

## 构建镜像
* api
```bash
cd api
docker build -t ziwei/api .
```
* ui
编辑src/http.ts，修改baseUrl
```bash
const baseUrl="http(s)://your_domain"
docker build -t ziwei/ui .
```

## 部署
http访问，cert-manager.io/cluster-issuer 注解可以不用设置。

如果启用https访问，将ingress.tls设置为true，
```bash
helm install ziwei chart \
  --namespace ziwei \
  --create-namespace \
  --set ingress.enabled=true \
  --set ingress.className=nginx \
  --set ingress.hostname=your_hostname \
  --set ingress.tls=false \
  --set ingress.annotations."cert-manager\.io/cluster-issuer"=your_issuer
```
