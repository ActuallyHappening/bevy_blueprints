cfg_if::cfg_if!(
	if #[cfg(not(feature = "macros"))] {
		
	} else {

	}
);