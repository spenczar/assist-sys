#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_create_simulation() {
        unsafe {
            let sim = reb_simulation_create();
            assert!(!sim.is_null());
            reb_simulation_free(sim);
        }
    }

    #[test]
    fn test_integrate_particle() {
        unsafe {
            let sim = reb_simulation_create();

	    let planets_file_path = CString::new("testdata/planets.dat").unwrap();
	    let asteroids_file_path = CString::new("testdata/asteroids.dat").unwrap();
            let ephem = assist_ephem_create(planets_file_path.into_raw(), asteroids_file_path.into_raw());
	    (*ephem).jd_ref = 2451545.0;

	    (*sim).t = 7304.5;
	    (*sim).ri_ias15.min_dt = 0.5;
            let ax = assist_attach(sim, ephem);

            // // These are the barycentric coordinates of asteroid Holman.
            // reb_simulation_add_fmt(r, "x y z vx vy vz",
            //     3.3388753502614090e+00,   // x in AU
            //     -9.1765182678903168e-01,  // y
            //     -5.0385906775843303e-01,  // z
            //     2.8056633153049852e-03,   // vx in AU/day
            //     7.5504086883996860e-03,   // vy
            //     2.9800282074358684e-03);  // vz
            let fmt_str = CString::new("x y z vx vy vz").unwrap();
            let particle = reb_particle_from_fmt(
                sim,
                fmt_str.as_ptr(),
                3.3388753502614090e+00,
                -9.1765182678903168e-01,
                -5.0385906775843303e-01,
                2.8056633153049852e-03,
                7.5504086883996860e-03,
                2.9800282074358684e-03,
            );

            reb_simulation_add(sim, particle);
	    
            let tend = 7404.5;
            while ((*sim).t < tend) && ((*sim).status <= 0) {
                reb_simulation_integrate(sim, (*sim).t + 20.0);

                let p = (*sim).particles;
                println!(
                    "particles[0]:  \tx = {:.12} \ty = {:.12} \tz = {:.12}",
                    (*p).x, (*p).y, (*p).z
                );
            }

	    assist_free(ax);
	    assist_ephem_free(ephem);
	    reb_simulation_free(sim);
        }
    }
}
