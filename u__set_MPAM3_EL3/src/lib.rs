#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use MPAM3_EL3_write::*;
use u_get_MPAM3_EL3_Type_MPAMEN::*;
use MPAM3_EL3_read::*;
use u_update_MPAM3_EL3_Type_MPAMEN::*;
use Mk_MPAM3_EL3_Type::*;
use common::*;
pub fn u__set_MPAM3_EL3<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        ga_26265: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call MPAM3_EL3_read(s_0_2)
        let s_0_3: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_0_2);
        // D s_0_4: write-var ga#26265 <= s_0_3
        fn_state.ga_26265 = s_0_3;
        // D s_0_5: read-var ga#26265.0:struct
        let s_0_5: u64 = fn_state.ga_26265._0;
        // D s_0_6: read-var r.0:struct
        let s_0_6: u64 = fn_state.r._0;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #63s : i
        let s_0_8: i128 = 63;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 64u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u63
        let s_0_11: u64 = (s_0_10.value() as u64);
        // C s_0_12: const #63s : i
        let s_0_12: i128 = 63;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: cast zx s_0_5 -> bv
        let s_0_14: Bits = Bits::new(s_0_5 as u128, 64u16);
        // D s_0_15: cast zx s_0_11 -> bv
        let s_0_15: Bits = Bits::new(s_0_11 as u128, 63u16);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_12
        let s_0_18: Bits = s_0_17 << s_0_12;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_15 s_0_19
        let s_0_20: Bits = ((s_0_15) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_13
        let s_0_21: Bits = s_0_20 << s_0_13;
        // C s_0_22: lsl s_0_19 s_0_13
        let s_0_22: Bits = s_0_19 << s_0_13;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_14 s_0_23
        let s_0_24: Bits = ((s_0_14) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u64
        let s_0_26: u64 = (s_0_25.value() as u64);
        // D s_0_27: call Mk_MPAM3_EL3_Type(s_0_26)
        let s_0_27: ProductType5c790c8ef59cc8b2 = Mk_MPAM3_EL3_Type(
            state,
            tracer,
            s_0_26,
        );
        // D s_0_28: call MPAM3_EL3_write(s_0_27)
        let s_0_28: () = MPAM3_EL3_write(state, tracer, s_0_27);
        // C s_0_29: const #() : ()
        let s_0_29: () = ();
        // S s_0_30: call MPAM3_EL3_read(s_0_29)
        let s_0_30: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_0_29);
        // D s_0_31: call _get_MPAM3_EL3_Type_MPAMEN(s_0_0)
        let s_0_31: bool = u_get_MPAM3_EL3_Type_MPAMEN(state, tracer, s_0_0);
        // D s_0_32: call _update_MPAM3_EL3_Type_MPAMEN(s_0_30, s_0_31)
        let s_0_32: ProductType5c790c8ef59cc8b2 = u_update_MPAM3_EL3_Type_MPAMEN(
            state,
            tracer,
            s_0_30,
            s_0_31,
        );
        // D s_0_33: call MPAM3_EL3_write(s_0_32)
        let s_0_33: () = MPAM3_EL3_write(state, tracer, s_0_32);
        // N s_0_34: return
        return;
    }
}
