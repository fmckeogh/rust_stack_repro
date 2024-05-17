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
use u_get_FPSR_Type_N::*;
use CheckVFPEnabled::*;
use R_set::*;
use MVFR1_read::*;
use u_get_FPSR_Type_V::*;
use FPEXC_read::*;
use AArch32_CheckAdvSIMDOrFPRegisterTraps::*;
use u_get_FPSR_Type_C::*;
use FPSCR_read__1::*;
use MVFR0_read::*;
use u_get_FPSR_Type_Z::*;
use MVFR2_read::*;
use Unreachable::*;
use common::*;
pub fn execute_aarch32_instrs_VMRS_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    reg: u8,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_364356: ProductType700c18a878c5601b,
        ga_364354: ProductType700c18a878c5601b,
        ga_364346: ProductType700c18a878c5601b,
        ga_364352: ProductType700c18a878c5601b,
        ga_364358: ProductType700c18a878c5601b,
        reg: u8,
        t: i64,
    }
    let fn_state = FunctionState {
        reg,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var reg:u8
        let s_0_0: u8 = fn_state.reg;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #1u : u8
        let s_0_2: u8 = 1;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b16 b1
        if s_0_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b15 b2
        if s_1_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // S s_2_1: call CheckVFPEnabled(s_2_0)
        let s_2_1: () = CheckVFPEnabled(state, tracer, s_2_0);
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var reg:u8
        let s_3_0: u8 = fn_state.reg;
        // D s_3_1: call AArch32_CheckAdvSIMDOrFPRegisterTraps(s_3_0)
        let s_3_1: () = AArch32_CheckAdvSIMDOrFPRegisterTraps(state, tracer, s_3_0);
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var reg:u8
        let s_4_0: u8 = fn_state.reg;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_2: const #0u : u8
        let s_4_2: u8 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #15576u : u32
        let s_5_0: u32 = 15576;
        // D s_5_1: read-reg s_5_0:u32
        let s_5_1: u32 = {
            let value = state.read_register::<u32>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: read-var t:i64
        let s_5_2: i64 = fn_state.t;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: call R_set(s_5_3, s_5_1)
        let s_5_4: () = R_set(state, tracer, s_5_3, s_5_1);
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var reg:u8
        let s_6_0: u8 = fn_state.reg;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_2: const #5u : u8
        let s_6_2: u8 = 5;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call MVFR2_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = MVFR2_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#364352 <= s_7_1
        fn_state.ga_364352 = s_7_1;
        // D s_7_3: read-var ga#364352.0:struct
        let s_7_3: u32 = fn_state.ga_364352._0;
        // D s_7_4: read-var t:i64
        let s_7_4: i64 = fn_state.t;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: call R_set(s_7_5, s_7_3)
        let s_7_6: () = R_set(state, tracer, s_7_5, s_7_3);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var reg:u8
        let s_8_0: u8 = fn_state.reg;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_2: const #6u : u8
        let s_8_2: u8 = 6;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 4u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call MVFR1_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = MVFR1_read(state, tracer, s_9_0);
        // D s_9_2: write-var ga#364354 <= s_9_1
        fn_state.ga_364354 = s_9_1;
        // D s_9_3: read-var ga#364354.0:struct
        let s_9_3: u32 = fn_state.ga_364354._0;
        // D s_9_4: read-var t:i64
        let s_9_4: i64 = fn_state.t;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: call R_set(s_9_5, s_9_3)
        let s_9_6: () = R_set(state, tracer, s_9_5, s_9_3);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var reg:u8
        let s_10_0: u8 = fn_state.reg;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 4u16);
        // C s_10_2: const #7u : u8
        let s_10_2: u8 = 7;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call MVFR0_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = MVFR0_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#364356 <= s_11_1
        fn_state.ga_364356 = s_11_1;
        // D s_11_3: read-var ga#364356.0:struct
        let s_11_3: u32 = fn_state.ga_364356._0;
        // D s_11_4: read-var t:i64
        let s_11_4: i64 = fn_state.t;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: call R_set(s_11_5, s_11_3)
        let s_11_6: () = R_set(state, tracer, s_11_5, s_11_3);
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var reg:u8
        let s_12_0: u8 = fn_state.reg;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 4u16);
        // C s_12_2: const #8u : u8
        let s_12_2: u8 = 8;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 4u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call FPEXC_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_13_0);
        // D s_13_2: write-var ga#364358 <= s_13_1
        fn_state.ga_364358 = s_13_1;
        // D s_13_3: read-var ga#364358.0:struct
        let s_13_3: u32 = fn_state.ga_364358._0;
        // D s_13_4: read-var t:i64
        let s_13_4: i64 = fn_state.t;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: call R_set(s_13_5, s_13_3)
        let s_13_6: () = R_set(state, tracer, s_13_5, s_13_3);
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Unreachable(s_14_0)
        let s_14_1: () = Unreachable(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // S s_16_1: call CheckVFPEnabled(s_16_0)
        let s_16_1: () = CheckVFPEnabled(state, tracer, s_16_0);
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #15s : i
        let s_17_0: i128 = 15;
        // D s_17_1: read-var t:i64
        let s_17_1: i64 = fn_state.t;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cmp-eq s_17_2 s_17_0
        let s_17_3: bool = ((s_17_2) == (s_17_0));
        // N s_17_4: branch s_17_3 b19 b18
        if s_17_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call FPSCR_read__1(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_18_0);
        // D s_18_2: write-var ga#364346 <= s_18_1
        fn_state.ga_364346 = s_18_1;
        // D s_18_3: read-var ga#364346.0:struct
        let s_18_3: u32 = fn_state.ga_364346._0;
        // D s_18_4: read-var t:i64
        let s_18_4: i64 = fn_state.t;
        // D s_18_5: cast zx s_18_4 -> i
        let s_18_5: i128 = (i128::try_from(s_18_4).unwrap());
        // D s_18_6: call R_set(s_18_5, s_18_3)
        let s_18_6: () = R_set(state, tracer, s_18_5, s_18_3);
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #20696u : u32
        let s_19_0: u32 = 20696;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_FPSR_Type_N(s_19_1)
        let s_19_2: bool = u_get_FPSR_Type_N(state, tracer, s_19_1);
        // C s_19_3: const #20696u : u32
        let s_19_3: u32 = 20696;
        // D s_19_4: read-reg s_19_3:struct
        let s_19_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: call _get_FPSR_Type_Z(s_19_4)
        let s_19_5: bool = u_get_FPSR_Type_Z(state, tracer, s_19_4);
        // C s_19_6: const #20696u : u32
        let s_19_6: u32 = 20696;
        // D s_19_7: read-reg s_19_6:struct
        let s_19_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_6 as isize);
            tracer.read_register(s_19_6 as isize, value);
            value
        };
        // D s_19_8: call _get_FPSR_Type_C(s_19_7)
        let s_19_8: bool = u_get_FPSR_Type_C(state, tracer, s_19_7);
        // C s_19_9: const #20696u : u32
        let s_19_9: u32 = 20696;
        // D s_19_10: read-reg s_19_9:struct
        let s_19_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_9 as isize);
            tracer.read_register(s_19_9 as isize, value);
            value
        };
        // D s_19_11: call _get_FPSR_Type_V(s_19_10)
        let s_19_11: bool = u_get_FPSR_Type_V(state, tracer, s_19_10);
        // D s_19_12: cast zx s_19_8 -> bv
        let s_19_12: Bits = Bits::new(s_19_8 as u128, 1u16);
        // D s_19_13: cast zx s_19_11 -> bv
        let s_19_13: Bits = Bits::new(s_19_11 as u128, 1u16);
        // D s_19_14: cast reint s_19_12 -> u128
        let s_19_14: u128 = (s_19_12.value() as u128);
        // D s_19_15: size-of s_19_12
        let s_19_15: u16 = s_19_12.length();
        // D s_19_16: cast reint s_19_13 -> u128
        let s_19_16: u128 = (s_19_13.value() as u128);
        // D s_19_17: size-of s_19_13
        let s_19_17: u16 = s_19_13.length();
        // D s_19_18: lsl s_19_14 s_19_17
        let s_19_18: u128 = s_19_14 << s_19_17;
        // D s_19_19: or s_19_18 s_19_16
        let s_19_19: u128 = ((s_19_18) | (s_19_16));
        // D s_19_20: add s_19_15 s_19_17
        let s_19_20: u16 = (s_19_15 + s_19_17);
        // D s_19_21: create-bits s_19_19 s_19_20
        let s_19_21: Bits = Bits::new(s_19_19, s_19_20);
        // D s_19_22: cast reint s_19_21 -> u8
        let s_19_22: u8 = (s_19_21.value() as u8);
        // D s_19_23: cast zx s_19_5 -> bv
        let s_19_23: Bits = Bits::new(s_19_5 as u128, 1u16);
        // D s_19_24: cast zx s_19_22 -> bv
        let s_19_24: Bits = Bits::new(s_19_22 as u128, 2u16);
        // D s_19_25: cast reint s_19_23 -> u128
        let s_19_25: u128 = (s_19_23.value() as u128);
        // D s_19_26: size-of s_19_23
        let s_19_26: u16 = s_19_23.length();
        // D s_19_27: cast reint s_19_24 -> u128
        let s_19_27: u128 = (s_19_24.value() as u128);
        // D s_19_28: size-of s_19_24
        let s_19_28: u16 = s_19_24.length();
        // D s_19_29: lsl s_19_25 s_19_28
        let s_19_29: u128 = s_19_25 << s_19_28;
        // D s_19_30: or s_19_29 s_19_27
        let s_19_30: u128 = ((s_19_29) | (s_19_27));
        // D s_19_31: add s_19_26 s_19_28
        let s_19_31: u16 = (s_19_26 + s_19_28);
        // D s_19_32: create-bits s_19_30 s_19_31
        let s_19_32: Bits = Bits::new(s_19_30, s_19_31);
        // D s_19_33: cast reint s_19_32 -> u8
        let s_19_33: u8 = (s_19_32.value() as u8);
        // D s_19_34: cast zx s_19_2 -> bv
        let s_19_34: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_35: cast zx s_19_33 -> bv
        let s_19_35: Bits = Bits::new(s_19_33 as u128, 3u16);
        // D s_19_36: cast reint s_19_34 -> u128
        let s_19_36: u128 = (s_19_34.value() as u128);
        // D s_19_37: size-of s_19_34
        let s_19_37: u16 = s_19_34.length();
        // D s_19_38: cast reint s_19_35 -> u128
        let s_19_38: u128 = (s_19_35.value() as u128);
        // D s_19_39: size-of s_19_35
        let s_19_39: u16 = s_19_35.length();
        // D s_19_40: lsl s_19_36 s_19_39
        let s_19_40: u128 = s_19_36 << s_19_39;
        // D s_19_41: or s_19_40 s_19_38
        let s_19_41: u128 = ((s_19_40) | (s_19_38));
        // D s_19_42: add s_19_37 s_19_39
        let s_19_42: u16 = (s_19_37 + s_19_39);
        // D s_19_43: create-bits s_19_41 s_19_42
        let s_19_43: Bits = Bits::new(s_19_41, s_19_42);
        // D s_19_44: cast reint s_19_43 -> u8
        let s_19_44: u8 = (s_19_43.value() as u8);
        // C s_19_45: const #3s : i
        let s_19_45: i128 = 3;
        // D s_19_46: cast zx s_19_44 -> bv
        let s_19_46: Bits = Bits::new(s_19_44 as u128, 4u16);
        // C s_19_47: const #1s : i64
        let s_19_47: i64 = 1;
        // C s_19_48: cast zx s_19_47 -> i
        let s_19_48: i128 = (i128::try_from(s_19_47).unwrap());
        // C s_19_49: const #0s : i
        let s_19_49: i128 = 0;
        // C s_19_50: add s_19_49 s_19_48
        let s_19_50: i128 = (s_19_49 + s_19_48);
        // D s_19_51: bit-extract s_19_46 s_19_45 s_19_50
        let s_19_51: Bits = (Bits::new(
            ((s_19_46) >> (s_19_45)).value(),
            u16::try_from(s_19_50).unwrap(),
        ));
        // D s_19_52: cast reint s_19_51 -> u8
        let s_19_52: bool = ((s_19_51.value()) != 0);
        // C s_19_53: const #16984u : u32
        let s_19_53: u32 = 16984;
        // N s_19_54: write-reg s_19_53 <= s_19_52
        let s_19_54: () = {
            state.write_register::<bool>(s_19_53 as isize, s_19_52);
            tracer.write_register(s_19_53 as isize, s_19_52);
        };
        // C s_19_55: const #2s : i
        let s_19_55: i128 = 2;
        // D s_19_56: cast zx s_19_44 -> bv
        let s_19_56: Bits = Bits::new(s_19_44 as u128, 4u16);
        // C s_19_57: const #1s : i64
        let s_19_57: i64 = 1;
        // C s_19_58: cast zx s_19_57 -> i
        let s_19_58: i128 = (i128::try_from(s_19_57).unwrap());
        // C s_19_59: const #0s : i
        let s_19_59: i128 = 0;
        // C s_19_60: add s_19_59 s_19_58
        let s_19_60: i128 = (s_19_59 + s_19_58);
        // D s_19_61: bit-extract s_19_56 s_19_55 s_19_60
        let s_19_61: Bits = (Bits::new(
            ((s_19_56) >> (s_19_55)).value(),
            u16::try_from(s_19_60).unwrap(),
        ));
        // D s_19_62: cast reint s_19_61 -> u8
        let s_19_62: bool = ((s_19_61.value()) != 0);
        // C s_19_63: const #16997u : u32
        let s_19_63: u32 = 16997;
        // N s_19_64: write-reg s_19_63 <= s_19_62
        let s_19_64: () = {
            state.write_register::<bool>(s_19_63 as isize, s_19_62);
            tracer.write_register(s_19_63 as isize, s_19_62);
        };
        // C s_19_65: const #1s : i
        let s_19_65: i128 = 1;
        // D s_19_66: cast zx s_19_44 -> bv
        let s_19_66: Bits = Bits::new(s_19_44 as u128, 4u16);
        // C s_19_67: const #1s : i64
        let s_19_67: i64 = 1;
        // C s_19_68: cast zx s_19_67 -> i
        let s_19_68: i128 = (i128::try_from(s_19_67).unwrap());
        // C s_19_69: const #0s : i
        let s_19_69: i128 = 0;
        // C s_19_70: add s_19_69 s_19_68
        let s_19_70: i128 = (s_19_69 + s_19_68);
        // D s_19_71: bit-extract s_19_66 s_19_65 s_19_70
        let s_19_71: Bits = (Bits::new(
            ((s_19_66) >> (s_19_65)).value(),
            u16::try_from(s_19_70).unwrap(),
        ));
        // D s_19_72: cast reint s_19_71 -> u8
        let s_19_72: bool = ((s_19_71.value()) != 0);
        // C s_19_73: const #16971u : u32
        let s_19_73: u32 = 16971;
        // N s_19_74: write-reg s_19_73 <= s_19_72
        let s_19_74: () = {
            state.write_register::<bool>(s_19_73 as isize, s_19_72);
            tracer.write_register(s_19_73 as isize, s_19_72);
        };
        // C s_19_75: const #0s : i
        let s_19_75: i128 = 0;
        // D s_19_76: cast zx s_19_44 -> bv
        let s_19_76: Bits = Bits::new(s_19_44 as u128, 4u16);
        // C s_19_77: const #1s : i64
        let s_19_77: i64 = 1;
        // C s_19_78: cast zx s_19_77 -> i
        let s_19_78: i128 = (i128::try_from(s_19_77).unwrap());
        // C s_19_79: const #0s : i
        let s_19_79: i128 = 0;
        // C s_19_80: add s_19_79 s_19_78
        let s_19_80: i128 = (s_19_79 + s_19_78);
        // D s_19_81: bit-extract s_19_76 s_19_75 s_19_80
        let s_19_81: Bits = (Bits::new(
            ((s_19_76) >> (s_19_75)).value(),
            u16::try_from(s_19_80).unwrap(),
        ));
        // D s_19_82: cast reint s_19_81 -> u8
        let s_19_82: bool = ((s_19_81.value()) != 0);
        // C s_19_83: const #16996u : u32
        let s_19_83: u32 = 16996;
        // N s_19_84: write-reg s_19_83 <= s_19_82
        let s_19_84: () = {
            state.write_register::<bool>(s_19_83 as isize, s_19_82);
            tracer.write_register(s_19_83 as isize, s_19_82);
        };
        // N s_19_85: return
        return;
    }
}
