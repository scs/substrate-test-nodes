pipeline {
  agent {
    node {
      label 'rust&&sgx'
    }
  }
  options {
    timeout(time: 1, unit: 'HOURS')
    buildDiscarder(logRotator(numToKeepStr: '14'))
  }
  stages {
    stage('Environment') {
      steps {
        sh './ci/install_rust.sh'
      }
    }
    stage('Build') {
      steps {
        // sh 'cargo build --release --examples'
        sh 'ls -la > directories.txt'
      }
    }
    stage('Archive artifact') {
      steps {
        archiveArtifacts artifacts: 'directories.txt', fingerprint: true
      }
    }
  }
}
