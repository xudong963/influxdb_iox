# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class InfluxdbIox < Formula
  desc "InfluxDB IOx is the future core of InfluxDB, an open source time series database."
  homepage "https://github.com/influxdata/influxdb_iox"
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/influxdata/influxdb_iox.git", :branch => "main"

  # Rust version found in file influxdb_iox/rust-toolchain
#   depends_on "rustup" => [:build, :test]
  depends_on "rustup" => :build
  depends_on "flatbuffers" => :build

  def install
    # TODO option head
    # brew install --build-bottle --HEAD ./influxdb_iox.rb --verbose --debug
    system "rustup-init", "-y", "--no-modify-path", "--no-update-default-toolchain"
    system ENV["CARGO_HOME"]+"/bin/cargo", "install", *std_cargo_args

    # TODO option non-head release
  end

  test do
    # `test do` will create, run in and delete a temporary directory.
    #
    # This test will fail and we won't accept that! For Homebrew/homebrew-core
    # this will need to be a test that verifies the functionality of the
    # software. Run the test with `brew test influxdb_iox`. Options passed
    # to `brew install` such as `--HEAD` also need to be provided to `brew test`.
    #
    # The installed folder is not in the path, so use the entire path to any
    # executables being tested: `system "#{bin}/program", "do", "something"`.
    system "cargo", "test", *std_cargo_args
  end
end
