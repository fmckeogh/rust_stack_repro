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
use EffectiveSCR_EL3_NSE::*;
use u__UNKNOWN_bits::*;
use HaveAArch32::*;
use ELFromM32::*;
use HaveRME::*;
use IsSecureEL2Enabled::*;
use EffectiveSCR_EL3_NS::*;
use HaveAArch64::*;
use u_get_SCR_EL3_Type_NS::*;
use common::*;
pub fn ELFromSPSR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    spsr: Bits,
) -> ProductTypea5cc8de4daab131c {
    #[derive(Default)]
    struct FunctionState {
        gs_4764: bool,
        gs_4762: bool,
        ga_3172: ProductTypea5cc8de4daab131c,
        gs_4765: bool,
        gs_4763: bool,
        valid_name: bool,
        gs_4766: bool,
        gs_4767: bool,
        effective_nse_nsshadow_60: u8,
        el: u8,
        spsr: Bits,
    }
    let fn_state = FunctionState {
        spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_0_0: const #4s : i
        let s_0_0: i128 = 4;
        // D s_0_1: read-var spsr:bv
        let s_0_1: Bits = fn_state.spsr;
        // C s_0_2: const #1u : u64
        let s_0_2: u64 = 1;
        // D s_0_3: bit-extract s_0_1 s_0_0 s_0_2
        let s_0_3: Bits = (Bits::new(
            ((s_0_1) >> (s_0_0)).value(),
            u16::try_from(s_0_2).unwrap(),
        ));
        // D s_0_4: cast reint s_0_3 -> u8
        let s_0_4: bool = ((s_0_3.value()) != 0);
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // C s_0_6: const #0u : u64
        let s_0_6: u64 = 0;
        // D s_0_7: cast zx s_0_4 -> u64
        let s_0_7: u64 = (s_0_4 as u64);
        // C s_0_8: const #1u : u64
        let s_0_8: u64 = 1;
        // D s_0_9: and s_0_7 s_0_8
        let s_0_9: u64 = ((s_0_7) & (s_0_8));
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // D s_0_11: lsl s_0_7 s_0_5
        let s_0_11: u64 = s_0_7 << s_0_5;
        // D s_0_12: or s_0_6 s_0_11
        let s_0_12: u64 = ((s_0_6) | (s_0_11));
        // D s_0_13: cmpl s_0_11
        let s_0_13: u64 = !s_0_11;
        // D s_0_14: and s_0_6 s_0_13
        let s_0_14: u64 = ((s_0_6) & (s_0_13));
        // D s_0_15: select s_0_10 s_0_12 s_0_14
        let s_0_15: u64 = if s_0_10 { s_0_12 } else { s_0_14 };
        // D s_0_16: cast trunc s_0_15 -> u8
        let s_0_16: bool = ((s_0_15) != 0);
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // C s_0_18: const #0u : u8
        let s_0_18: bool = false;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // N s_0_21: branch s_0_20 b8 b1
        if s_0_20 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveAArch32(s_1_0)
        let s_1_1: bool = HaveAArch32(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b7 b2
        if s_1_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var valid_name <= s_2_0
        fn_state.valid_name = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_3_0: read-var valid_name:u8
        let s_3_0: bool = fn_state.valid_name;
        // D s_3_1: not s_3_0
        let s_3_1: bool = !s_3_0;
        // N s_3_2: branch s_3_1 b6 b4
        if s_3_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_5_0: read-var el:u8
        let s_5_0: u8 = fn_state.el;
        // D s_5_1: read-var valid_name:u8
        let s_5_1: bool = fn_state.valid_name;
        // D s_5_2: create-product struct = ["s_5_1", "s_5_0"]
        let s_5_2: ProductTypea5cc8de4daab131c = ProductTypea5cc8de4daab131c {
            _0: s_5_1,
            _1: s_5_0,
        };
        // N s_5_3: return s_5_2
        return s_5_2;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // C s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // S s_6_2: call __UNKNOWN_bits(s_6_1)
        let s_6_2: Bits = u__UNKNOWN_bits(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u8
        let s_6_3: u8 = (s_6_2.value() as u8);
        // D s_6_4: write-var el <= s_6_3
        fn_state.el = s_6_3;
        // N s_6_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var spsr:bv
        let s_7_1: Bits = fn_state.spsr;
        // C s_7_2: const #1s : i64
        let s_7_2: i64 = 1;
        // C s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // C s_7_4: const #4s : i
        let s_7_4: i128 = 4;
        // C s_7_5: add s_7_4 s_7_3
        let s_7_5: i128 = (s_7_4 + s_7_3);
        // D s_7_6: bit-extract s_7_1 s_7_0 s_7_5
        let s_7_6: Bits = (Bits::new(
            ((s_7_1) >> (s_7_0)).value(),
            u16::try_from(s_7_5).unwrap(),
        ));
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: u8 = (s_7_6.value() as u8);
        // D s_7_8: call ELFromM32(s_7_7)
        let s_7_8: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_7_7);
        // D s_7_9: write-var ga#3172 <= s_7_8
        fn_state.ga_3172 = s_7_8;
        // D s_7_10: read-var ga#3172.0:struct
        let s_7_10: bool = fn_state.ga_3172._0;
        // D s_7_11: read-var ga#3172.1:struct
        let s_7_11: u8 = fn_state.ga_3172._1;
        // D s_7_12: write-var valid_name <= s_7_10
        fn_state.valid_name = s_7_10;
        // D s_7_13: write-var el <= s_7_11
        fn_state.el = s_7_11;
        // N s_7_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // D s_8_1: read-var spsr:bv
        let s_8_1: Bits = fn_state.spsr;
        // C s_8_2: const #1s : i64
        let s_8_2: i64 = 1;
        // C s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: const #1s : i
        let s_8_4: i128 = 1;
        // C s_8_5: add s_8_4 s_8_3
        let s_8_5: i128 = (s_8_4 + s_8_3);
        // D s_8_6: bit-extract s_8_1 s_8_0 s_8_5
        let s_8_6: Bits = (Bits::new(
            ((s_8_1) >> (s_8_0)).value(),
            u16::try_from(s_8_5).unwrap(),
        ));
        // D s_8_7: cast reint s_8_6 -> u8
        let s_8_7: u8 = (s_8_6.value() as u8);
        // D s_8_8: write-var el <= s_8_7
        fn_state.el = s_8_7;
        // C s_8_9: const #() : ()
        let s_8_9: () = ();
        // S s_8_10: call EffectiveSCR_EL3_NSE(s_8_9)
        let s_8_10: bool = EffectiveSCR_EL3_NSE(state, tracer, s_8_9);
        // C s_8_11: const #() : ()
        let s_8_11: () = ();
        // S s_8_12: call EffectiveSCR_EL3_NS(s_8_11)
        let s_8_12: bool = EffectiveSCR_EL3_NS(state, tracer, s_8_11);
        // S s_8_13: cast zx s_8_10 -> bv
        let s_8_13: Bits = Bits::new(s_8_10 as u128, 1u16);
        // S s_8_14: cast zx s_8_12 -> bv
        let s_8_14: Bits = Bits::new(s_8_12 as u128, 1u16);
        // S s_8_15: cast reint s_8_13 -> u128
        let s_8_15: u128 = (s_8_13.value() as u128);
        // D s_8_16: size-of s_8_13
        let s_8_16: u16 = s_8_13.length();
        // S s_8_17: cast reint s_8_14 -> u128
        let s_8_17: u128 = (s_8_14.value() as u128);
        // D s_8_18: size-of s_8_14
        let s_8_18: u16 = s_8_14.length();
        // D s_8_19: lsl s_8_15 s_8_18
        let s_8_19: u128 = s_8_15 << s_8_18;
        // D s_8_20: or s_8_19 s_8_17
        let s_8_20: u128 = ((s_8_19) | (s_8_17));
        // D s_8_21: add s_8_16 s_8_18
        let s_8_21: u16 = (s_8_16 + s_8_18);
        // D s_8_22: create-bits s_8_20 s_8_21
        let s_8_22: Bits = Bits::new(s_8_20, s_8_21);
        // D s_8_23: cast reint s_8_22 -> u8
        let s_8_23: u8 = (s_8_22.value() as u8);
        // D s_8_24: write-var effective_nse_nsshadow#60 <= s_8_23
        fn_state.effective_nse_nsshadow_60 = s_8_23;
        // C s_8_25: const #() : ()
        let s_8_25: () = ();
        // S s_8_26: call HaveAArch64(s_8_25)
        let s_8_26: bool = HaveAArch64(state, tracer, s_8_25);
        // S s_8_27: not s_8_26
        let s_8_27: bool = !s_8_26;
        // N s_8_28: branch s_8_27 b38 b9
        if s_8_27 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_9_0: read-var el:u8
        let s_9_0: u8 = fn_state.el;
        // C s_9_1: const #2u : u8
        let s_9_1: u8 = 2;
        // D s_9_2: cmp-lt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) < (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b37 b10
        if s_9_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var spsr:bv
        let s_10_1: Bits = fn_state.spsr;
        // C s_10_2: const #1u : u64
        let s_10_2: u64 = 1;
        // D s_10_3: bit-extract s_10_1 s_10_0 s_10_2
        let s_10_3: Bits = (Bits::new(
            ((s_10_1) >> (s_10_0)).value(),
            u16::try_from(s_10_2).unwrap(),
        ));
        // D s_10_4: cast reint s_10_3 -> u8
        let s_10_4: bool = ((s_10_3.value()) != 0);
        // C s_10_5: const #0s : i
        let s_10_5: i128 = 0;
        // C s_10_6: const #0u : u64
        let s_10_6: u64 = 0;
        // D s_10_7: cast zx s_10_4 -> u64
        let s_10_7: u64 = (s_10_4 as u64);
        // C s_10_8: const #1u : u64
        let s_10_8: u64 = 1;
        // D s_10_9: and s_10_7 s_10_8
        let s_10_9: u64 = ((s_10_7) & (s_10_8));
        // D s_10_10: cmp-eq s_10_9 s_10_8
        let s_10_10: bool = ((s_10_9) == (s_10_8));
        // D s_10_11: lsl s_10_7 s_10_5
        let s_10_11: u64 = s_10_7 << s_10_5;
        // D s_10_12: or s_10_6 s_10_11
        let s_10_12: u64 = ((s_10_6) | (s_10_11));
        // D s_10_13: cmpl s_10_11
        let s_10_13: u64 = !s_10_11;
        // D s_10_14: and s_10_6 s_10_13
        let s_10_14: u64 = ((s_10_6) & (s_10_13));
        // D s_10_15: select s_10_10 s_10_12 s_10_14
        let s_10_15: u64 = if s_10_10 { s_10_12 } else { s_10_14 };
        // D s_10_16: cast trunc s_10_15 -> u8
        let s_10_16: bool = ((s_10_15) != 0);
        // D s_10_17: cast zx s_10_16 -> bv
        let s_10_17: Bits = Bits::new(s_10_16 as u128, 1u16);
        // C s_10_18: const #1u : u8
        let s_10_18: bool = true;
        // C s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 1u16);
        // D s_10_20: cmp-eq s_10_17 s_10_19
        let s_10_20: bool = ((s_10_17) == (s_10_19));
        // N s_10_21: branch s_10_20 b36 b11
        if s_10_20 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_11_0: read-var el:u8
        let s_11_0: u8 = fn_state.el;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #448u : u32
        let s_11_2: u32 = 448;
        // D s_11_3: read-reg s_11_2:u8
        let s_11_3: u8 = {
            let value = state.read_register::<u8>(s_11_2 as isize);
            tracer.read_register(s_11_2 as isize, value);
            value
        };
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 2u16);
        // D s_11_5: cmp-eq s_11_1 s_11_4
        let s_11_5: bool = ((s_11_1) == (s_11_4));
        // N s_11_6: branch s_11_5 b35 b12
        if s_11_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#4762 <= s_12_0
        fn_state.gs_4762 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_13_0: read-var gs#4762:u8
        let s_13_0: bool = fn_state.gs_4762;
        // N s_13_1: branch s_13_0 b34 b14
        if s_13_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveRME(s_14_0)
        let s_14_1: bool = HaveRME(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b33 b15
        if s_14_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#4763 <= s_15_0
        fn_state.gs_4763 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_16_0: read-var gs#4763:u8
        let s_16_0: bool = fn_state.gs_4763;
        // N s_16_1: branch s_16_0 b32 b17
        if s_16_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#4764 <= s_17_0
        fn_state.gs_4764 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_18_0: read-var gs#4764:u8
        let s_18_0: bool = fn_state.gs_4764;
        // N s_18_1: branch s_18_0 b31 b19
        if s_18_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_19_0: read-var el:u8
        let s_19_0: u8 = fn_state.el;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #432u : u32
        let s_19_2: u32 = 432;
        // D s_19_3: read-reg s_19_2:u8
        let s_19_3: u8 = {
            let value = state.read_register::<u8>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 2u16);
        // D s_19_5: cmp-eq s_19_1 s_19_4
        let s_19_5: bool = ((s_19_1) == (s_19_4));
        // N s_19_6: branch s_19_5 b30 b20
        if s_19_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#4765 <= s_20_0
        fn_state.gs_4765 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_21_0: read-var gs#4765:u8
        let s_21_0: bool = fn_state.gs_4765;
        // N s_21_1: branch s_21_0 b29 b22
        if s_21_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#4766 <= s_22_0
        fn_state.gs_4766 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_23_0: read-var gs#4766:u8
        let s_23_0: bool = fn_state.gs_4766;
        // N s_23_1: branch s_23_0 b28 b24
        if s_23_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#4767 <= s_24_0
        fn_state.gs_4767 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_25_0: read-var gs#4767:u8
        let s_25_0: bool = fn_state.gs_4767;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var valid_name <= s_26_0
        fn_state.valid_name = s_26_0;
        // N s_26_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var valid_name <= s_27_0
        fn_state.valid_name = s_27_0;
        // N s_27_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_28_0: const #90704u : u32
        let s_28_0: u32 = 90704;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_SCR_EL3_Type_NS(s_28_1)
        let s_28_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #0u : u8
        let s_28_4: bool = false;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#4767 <= s_28_6
        fn_state.gs_4767 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call IsSecureEL2Enabled(s_29_0)
        let s_29_1: bool = IsSecureEL2Enabled(state, tracer, s_29_0);
        // S s_29_2: not s_29_1
        let s_29_2: bool = !s_29_1;
        // D s_29_3: write-var gs#4766 <= s_29_2
        fn_state.gs_4766 = s_29_2;
        // N s_29_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // D s_30_4: write-var gs#4765 <= s_30_3
        fn_state.gs_4765 = s_30_3;
        // N s_30_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var valid_name <= s_31_0
        fn_state.valid_name = s_31_0;
        // N s_31_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_32_0: read-var effective_nse_nsshadow#60:u8
        let s_32_0: u8 = fn_state.effective_nse_nsshadow_60;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 2u16);
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 2u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#4764 <= s_32_4
        fn_state.gs_4764 = s_32_4;
        // N s_32_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_33_0: read-var el:u8
        let s_33_0: u8 = fn_state.el;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 2u16);
        // C s_33_2: const #424u : u32
        let s_33_2: u32 = 424;
        // D s_33_3: read-reg s_33_2:u8
        let s_33_3: u8 = {
            let value = state.read_register::<u8>(s_33_2 as isize);
            tracer.read_register(s_33_2 as isize, value);
            value
        };
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 2u16);
        // D s_33_5: cmp-ne s_33_1 s_33_4
        let s_33_5: bool = ((s_33_1) != (s_33_4));
        // D s_33_6: write-var gs#4763 <= s_33_5
        fn_state.gs_4763 = s_33_5;
        // N s_33_7: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var valid_name <= s_34_0
        fn_state.valid_name = s_34_0;
        // N s_34_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_35_0: const #0s : i
        let s_35_0: i128 = 0;
        // D s_35_1: read-var spsr:bv
        let s_35_1: Bits = fn_state.spsr;
        // C s_35_2: const #1u : u64
        let s_35_2: u64 = 1;
        // D s_35_3: bit-extract s_35_1 s_35_0 s_35_2
        let s_35_3: Bits = (Bits::new(
            ((s_35_1) >> (s_35_0)).value(),
            u16::try_from(s_35_2).unwrap(),
        ));
        // D s_35_4: cast reint s_35_3 -> u8
        let s_35_4: bool = ((s_35_3.value()) != 0);
        // C s_35_5: const #0s : i
        let s_35_5: i128 = 0;
        // C s_35_6: const #0u : u64
        let s_35_6: u64 = 0;
        // D s_35_7: cast zx s_35_4 -> u64
        let s_35_7: u64 = (s_35_4 as u64);
        // C s_35_8: const #1u : u64
        let s_35_8: u64 = 1;
        // D s_35_9: and s_35_7 s_35_8
        let s_35_9: u64 = ((s_35_7) & (s_35_8));
        // D s_35_10: cmp-eq s_35_9 s_35_8
        let s_35_10: bool = ((s_35_9) == (s_35_8));
        // D s_35_11: lsl s_35_7 s_35_5
        let s_35_11: u64 = s_35_7 << s_35_5;
        // D s_35_12: or s_35_6 s_35_11
        let s_35_12: u64 = ((s_35_6) | (s_35_11));
        // D s_35_13: cmpl s_35_11
        let s_35_13: u64 = !s_35_11;
        // D s_35_14: and s_35_6 s_35_13
        let s_35_14: u64 = ((s_35_6) & (s_35_13));
        // D s_35_15: select s_35_10 s_35_12 s_35_14
        let s_35_15: u64 = if s_35_10 { s_35_12 } else { s_35_14 };
        // D s_35_16: cast trunc s_35_15 -> u8
        let s_35_16: bool = ((s_35_15) != 0);
        // D s_35_17: cast zx s_35_16 -> bv
        let s_35_17: Bits = Bits::new(s_35_16 as u128, 1u16);
        // C s_35_18: const #1u : u8
        let s_35_18: bool = true;
        // C s_35_19: cast zx s_35_18 -> bv
        let s_35_19: Bits = Bits::new(s_35_18 as u128, 1u16);
        // D s_35_20: cmp-eq s_35_17 s_35_19
        let s_35_20: bool = ((s_35_17) == (s_35_19));
        // D s_35_21: write-var gs#4762 <= s_35_20
        fn_state.gs_4762 = s_35_20;
        // N s_35_22: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var valid_name <= s_36_0
        fn_state.valid_name = s_36_0;
        // N s_36_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var valid_name <= s_37_0
        fn_state.valid_name = s_37_0;
        // N s_37_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var valid_name <= s_38_0
        fn_state.valid_name = s_38_0;
        // N s_38_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
