///modules for a bar and foo
// reference privately
mod bar;

mod foo;

/// this is using bar::bar to be changed to barr instead.
pub use bar::bar as barr;

/// and this is just a public function for foofoo to become afoo
pub fn foofoo() {
	foo::afoo();
}
