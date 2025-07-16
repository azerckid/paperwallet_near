# 🐳 로컬에서 Docker로 Supabase 실행하기

이 문서는 `supabase-cli`와 Docker를 사용하여 로컬 컴퓨터에 Supabase 개발 환경을 구축하는 방법을 안내합니다.

## 왜 로컬 환경을 사용하나요?

- **비용 절감**: 클라우드 요금제와 상관없이 무제한으로 테스트할 수 있습니다.
- **빠른 개발 속도**: 코드를 변경하고 즉시 결과를 확인할 수 있어 개발 사이클이 단축됩니다.
- **안전한 실험**: 운영 데이터에 영향을 줄 걱정 없이 마음껏 스키마를 변경하고 기능을 테스트할 수 있습니다.

---

## Step 1: Docker 설치 및 실행

`supabase-cli`는 내부적으로 Docker를 사용하여 Supabase 스택을 실행합니다.

1.  Docker Desktop 공식 홈페이지에서 본인의 운영체제에 맞는 버전을 다운로드하여 설치합니다.
2.  **가장 중요한 단계**: Supabase CLI를 사용하기 전에 **Docker Desktop 애플리케이션을 반드시 실행**해야 합니다. Docker가 실행 중이지 않으면 CLI 명령어가 작동하지 않습니다.

---

## Step 2: Supabase CLI 설치

터미널을 열고 아래 명령어를 실행하여 `supabase-cli`를 설치합니다.

```bash
# npm을 사용하는 경우
npm install -g supabase

# 또는 Homebrew를 사용하는 경우 (Mac)
brew install supabase/tap/supabase-cli
```

---

## Step 3: Supabase 프로젝트 초기화

원하는 위치에 새 프로젝트 폴더를 만들고, 그 안에서 Supabase 프로젝트를 초기화합니다.

```bash
mkdir my-local-project && cd my-local-project

supabase init
```

이 명령어를 실행하면 현재 디렉토리에 `supabase`라는 폴더가 생성됩니다. 이 폴더에는 향후 데이터베이스 마이그레이션 파일이나 엣지 함수 코드 등이 저장됩니다.

---

## Step 4: 로컬 Supabase 실행하기

이제 Docker를 통해 로컬에 Supabase 서비스를 실행할 차례입니다.

```bash
supabase start
```

이 명령어를 실행하면 Docker가 필요한 이미지들을 다운로드하고 여러 개의 컨테이너를 실행합니다. 처음 실행 시에는 시간이 다소 걸릴 수 있습니다.

성공적으로 실행되면 터미널에 다음과 같이 중요한 정보들이 출력됩니다.

```plaintext
Started Supabase local development setup.

API URL: http://localhost:54321
DB URL: postgresql://postgres:postgres@localhost:54322/postgres
Studio URL: http://localhost:54323
Inbucket URL: http://localhost:54324
API anon key: eyJhbGciOiJI...
API service_role key: eyJhbGciOiJI...
```

- **API URL / anon key**: 프론트엔드 애플리케이션에서 Supabase에 연결할 때 사용합니다.
- **Studio URL**: 브라우저에서 이 주소로 접속하면 클라우드 버전과 똑같은 Supabase 대시보드를 사용할 수 있습니다.

---

## Step 5: 서비스 중지 및 초기화

개발이 끝나면 아래 명령어로 실행 중인 Docker 컨테이너들을 중지할 수 있습니다.

```bash
# Supabase 서비스 중지 (데이터는 보존됨)
supabase stop

# 서비스를 중지하고 데이터베이스를 완전히 초기화하고 싶을 때
supabase stop --no-backup
```
