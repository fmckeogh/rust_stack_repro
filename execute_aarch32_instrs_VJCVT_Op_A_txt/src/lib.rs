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
use CheckVFPEnabled::*;
use Mk_FPSCR_Type::*;
use FPSCR_write::*;
use FPSCR_read::*;
use FPSCR_read__1::*;
use S_set::*;
use D_read::*;
use FPToFixedJS::*;
use common::*;
pub fn execute_aarch32_instrs_VJCVT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_366387: ProductType700c18a878c5601b,
        gs_916168: ProductTypef506aa96a892fe52,
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call D_read(s_1_1)
        let s_1_2: u64 = D_read(state, tracer, s_1_1);
        // C s_1_3: const #() : ()
        let s_1_3: () = ();
        // S s_1_4: call FPSCR_read(s_1_3)
        let s_1_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_1_3);
        // D s_1_5: cast zx s_1_2 -> bv
        let s_1_5: Bits = Bits::new(s_1_2 as u128, 64u16);
        // C s_1_6: const #32s : i64
        let s_1_6: i64 = 32;
        // C s_1_7: const #0u : u8
        let s_1_7: bool = false;
        // D s_1_8: call FPToFixedJS(s_1_5, s_1_4, s_1_7, s_1_6)
        let s_1_8: ProductTypef506aa96a892fe52 = FPToFixedJS(
            state,
            tracer,
            s_1_5,
            s_1_4,
            s_1_7,
            s_1_6,
        );
        // D s_1_9: write-var gs#916168 <= s_1_8
        fn_state.gs_916168 = s_1_8;
        // N s_1_10: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#916168.0:struct
        let s_2_0: Bits = fn_state.gs_916168._0;
        // D s_2_1: cast reint s_2_0 -> u32
        let s_2_1: u32 = (s_2_0.value() as u32);
        // D s_2_2: read-var gs#916168.1:struct
        let s_2_2: bool = fn_state.gs_916168._1;
        // C s_2_3: const #() : ()
        let s_2_3: () = ();
        // S s_2_4: call FPSCR_read__1(s_2_3)
        let s_2_4: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_2_3);
        // D s_2_5: write-var ga#366387 <= s_2_4
        fn_state.ga_366387 = s_2_4;
        // D s_2_6: read-var ga#366387.0:struct
        let s_2_6: u32 = fn_state.ga_366387._0;
        // C s_2_7: const #0u : u8
        let s_2_7: bool = false;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cast zx s_2_2 -> bv
        let s_2_9: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_10: cast reint s_2_8 -> u128
        let s_2_10: u128 = (s_2_8.value() as u128);
        // D s_2_11: size-of s_2_8
        let s_2_11: u16 = s_2_8.length();
        // D s_2_12: cast reint s_2_9 -> u128
        let s_2_12: u128 = (s_2_9.value() as u128);
        // D s_2_13: size-of s_2_9
        let s_2_13: u16 = s_2_9.length();
        // D s_2_14: lsl s_2_10 s_2_13
        let s_2_14: u128 = s_2_10 << s_2_13;
        // D s_2_15: or s_2_14 s_2_12
        let s_2_15: u128 = ((s_2_14) | (s_2_12));
        // D s_2_16: add s_2_11 s_2_13
        let s_2_16: u16 = (s_2_11 + s_2_13);
        // D s_2_17: create-bits s_2_15 s_2_16
        let s_2_17: Bits = Bits::new(s_2_15, s_2_16);
        // D s_2_18: cast reint s_2_17 -> u8
        let s_2_18: u8 = (s_2_17.value() as u8);
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 2u16);
        // C s_2_20: const #0u : u8
        let s_2_20: u8 = 0;
        // C s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 2u16);
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // C s_2_24: cast reint s_2_21 -> u128
        let s_2_24: u128 = (s_2_21.value() as u128);
        // D s_2_25: size-of s_2_21
        let s_2_25: u16 = s_2_21.length();
        // D s_2_26: lsl s_2_22 s_2_25
        let s_2_26: u128 = s_2_22 << s_2_25;
        // D s_2_27: or s_2_26 s_2_24
        let s_2_27: u128 = ((s_2_26) | (s_2_24));
        // D s_2_28: add s_2_23 s_2_25
        let s_2_28: u16 = (s_2_23 + s_2_25);
        // D s_2_29: create-bits s_2_27 s_2_28
        let s_2_29: Bits = Bits::new(s_2_27, s_2_28);
        // D s_2_30: cast reint s_2_29 -> u8
        let s_2_30: u8 = (s_2_29.value() as u8);
        // C s_2_31: const #28s : i
        let s_2_31: i128 = 28;
        // D s_2_32: cast zx s_2_6 -> bv
        let s_2_32: Bits = Bits::new(s_2_6 as u128, 32u16);
        // D s_2_33: cast zx s_2_30 -> bv
        let s_2_33: Bits = Bits::new(s_2_30 as u128, 4u16);
        // C s_2_34: const #3s : i
        let s_2_34: i128 = 3;
        // C s_2_35: const #1u : u64
        let s_2_35: u64 = 1;
        // C s_2_36: cast zx s_2_35 -> bv
        let s_2_36: Bits = Bits::new(s_2_35 as u128, 64u16);
        // C s_2_37: lsl s_2_36 s_2_34
        let s_2_37: Bits = s_2_36 << s_2_34;
        // C s_2_38: sub s_2_37 s_2_36
        let s_2_38: Bits = ((s_2_37) - (s_2_36));
        // D s_2_39: and s_2_33 s_2_38
        let s_2_39: Bits = ((s_2_33) & (s_2_38));
        // D s_2_40: lsl s_2_39 s_2_31
        let s_2_40: Bits = s_2_39 << s_2_31;
        // C s_2_41: lsl s_2_38 s_2_31
        let s_2_41: Bits = s_2_38 << s_2_31;
        // C s_2_42: cmpl s_2_41
        let s_2_42: Bits = !s_2_41;
        // D s_2_43: and s_2_32 s_2_42
        let s_2_43: Bits = ((s_2_32) & (s_2_42));
        // D s_2_44: or s_2_43 s_2_40
        let s_2_44: Bits = ((s_2_43) | (s_2_40));
        // D s_2_45: cast reint s_2_44 -> u32
        let s_2_45: u32 = (s_2_44.value() as u32);
        // D s_2_46: call Mk_FPSCR_Type(s_2_45)
        let s_2_46: ProductType700c18a878c5601b = Mk_FPSCR_Type(state, tracer, s_2_45);
        // D s_2_47: call FPSCR_write(s_2_46)
        let s_2_47: () = FPSCR_write(state, tracer, s_2_46);
        // D s_2_48: read-var d:i64
        let s_2_48: i64 = fn_state.d;
        // D s_2_49: cast zx s_2_48 -> i
        let s_2_49: i128 = (i128::try_from(s_2_48).unwrap());
        // D s_2_50: call S_set(s_2_49, s_2_1)
        let s_2_50: () = S_set(state, tracer, s_2_49, s_2_1);
        // N s_2_51: return
        return;
    }
}
