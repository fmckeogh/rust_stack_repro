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
use BadMode::*;
use EffectiveSCR_EL3_NSE::*;
use u__UNKNOWN_bits::*;
use HaveRME::*;
use EffectiveSCR_EL3_NS::*;
use u_get_SCRType_NS::*;
use HaveAArch64::*;
use u_get_SCR_Type_NS::*;
use SCR_GEN_read::*;
use common::*;
pub fn ELFromM32<T: Tracer>(
    state: &mut State,
    tracer: &T,
    mode: u8,
) -> ProductTypea5cc8de4daab131c {
    #[derive(Default)]
    struct FunctionState {
        gs_4718: bool,
        gs_4742: bool,
        gs_4721: bool,
        gs_4724: bool,
        valid_name: bool,
        gs_4736: bool,
        gs_4731: bool,
        gs_4722: bool,
        el: u8,
        effective_nse_ns: u8,
        gs_4727: bool,
        gs_4719: bool,
        gs_4715: bool,
        gs_4741: bool,
        gs_4730: bool,
        gs_4737: bool,
        gs_4725: bool,
        gs_4735: bool,
        gs_4728: bool,
        gs_4716: bool,
        mode: u8,
    }
    let fn_state = FunctionState {
        mode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_0_0: read-var mode:u8
        let s_0_0: u8 = fn_state.mode;
        // D s_0_1: call BadMode(s_0_0)
        let s_0_1: bool = BadMode(state, tracer, s_0_0);
        // D s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // D s_0_3: write-var valid_name <= s_0_2
        fn_state.valid_name = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call EffectiveSCR_EL3_NSE(s_0_4)
        let s_0_5: bool = EffectiveSCR_EL3_NSE(state, tracer, s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call EffectiveSCR_EL3_NS(s_0_6)
        let s_0_7: bool = EffectiveSCR_EL3_NS(state, tracer, s_0_6);
        // S s_0_8: cast zx s_0_5 -> bv
        let s_0_8: Bits = Bits::new(s_0_5 as u128, 1u16);
        // S s_0_9: cast zx s_0_7 -> bv
        let s_0_9: Bits = Bits::new(s_0_7 as u128, 1u16);
        // S s_0_10: cast reint s_0_8 -> u128
        let s_0_10: u128 = (s_0_8.value() as u128);
        // D s_0_11: size-of s_0_8
        let s_0_11: u16 = s_0_8.length();
        // S s_0_12: cast reint s_0_9 -> u128
        let s_0_12: u128 = (s_0_9.value() as u128);
        // D s_0_13: size-of s_0_9
        let s_0_13: u16 = s_0_9.length();
        // D s_0_14: lsl s_0_10 s_0_13
        let s_0_14: u128 = s_0_10 << s_0_13;
        // D s_0_15: or s_0_14 s_0_12
        let s_0_15: u128 = ((s_0_14) | (s_0_12));
        // D s_0_16: add s_0_11 s_0_13
        let s_0_16: u16 = (s_0_11 + s_0_13);
        // D s_0_17: create-bits s_0_15 s_0_16
        let s_0_17: Bits = Bits::new(s_0_15, s_0_16);
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // D s_0_19: write-var effective_nse_ns <= s_0_18
        fn_state.effective_nse_ns = s_0_18;
        // D s_0_20: read-var mode:u8
        let s_0_20: u8 = fn_state.mode;
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 5u16);
        // C s_0_22: const #384u : u32
        let s_0_22: u32 = 384;
        // D s_0_23: read-reg s_0_22:u8
        let s_0_23: u8 = {
            let value = state.read_register::<u8>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 5u16);
        // D s_0_25: cmp-eq s_0_21 s_0_24
        let s_0_25: bool = ((s_0_21) == (s_0_24));
        // D s_0_26: not s_0_25
        let s_0_26: bool = !s_0_25;
        // N s_0_27: branch s_0_26 b26 b1
        if s_0_26 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var el <= s_1_1
        fn_state.el = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_2_0: read-var valid_name:u8
        let s_2_0: bool = fn_state.valid_name;
        // N s_2_1: branch s_2_0 b25 b3
        if s_2_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#4735 <= s_3_0
        fn_state.gs_4735 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_4_0: read-var gs#4735:u8
        let s_4_0: bool = fn_state.gs_4735;
        // N s_4_1: branch s_4_0 b24 b5
        if s_4_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#4736 <= s_5_0
        fn_state.gs_4736 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_6_0: read-var gs#4736:u8
        let s_6_0: bool = fn_state.gs_4736;
        // N s_6_1: branch s_6_0 b23 b7
        if s_6_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#4737 <= s_7_0
        fn_state.gs_4737 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_8_0: read-var gs#4737:u8
        let s_8_0: bool = fn_state.gs_4737;
        // N s_8_1: branch s_8_0 b22 b9
        if s_8_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_9_0: read-var valid_name:u8
        let s_9_0: bool = fn_state.valid_name;
        // N s_9_1: branch s_9_0 b21 b10
        if s_9_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#4741 <= s_10_0
        fn_state.gs_4741 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_11_0: read-var gs#4741:u8
        let s_11_0: bool = fn_state.gs_4741;
        // N s_11_1: branch s_11_0 b20 b12
        if s_11_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#4742 <= s_12_0
        fn_state.gs_4742 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_13_0: read-var gs#4742:u8
        let s_13_0: bool = fn_state.gs_4742;
        // N s_13_1: branch s_13_0 b19 b14
        if s_13_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_15_0: read-var valid_name:u8
        let s_15_0: bool = fn_state.valid_name;
        // D s_15_1: not s_15_0
        let s_15_1: bool = !s_15_0;
        // N s_15_2: branch s_15_1 b18 b16
        if s_15_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_17_0: read-var el:u8
        let s_17_0: u8 = fn_state.el;
        // D s_17_1: read-var valid_name:u8
        let s_17_1: bool = fn_state.valid_name;
        // D s_17_2: create-product struct = ["s_17_1", "s_17_0"]
        let s_17_2: ProductTypea5cc8de4daab131c = ProductTypea5cc8de4daab131c {
            _0: s_17_1,
            _1: s_17_0,
        };
        // N s_17_3: return s_17_2
        return s_17_2;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_18_0: const #2s : i64
        let s_18_0: i64 = 2;
        // C s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // S s_18_2: call __UNKNOWN_bits(s_18_1)
        let s_18_2: Bits = u__UNKNOWN_bits(state, tracer, s_18_1);
        // S s_18_3: cast reint s_18_2 -> u8
        let s_18_3: u8 = (s_18_2.value() as u8);
        // D s_18_4: write-var el <= s_18_3
        fn_state.el = s_18_3;
        // N s_18_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var valid_name <= s_19_0
        fn_state.valid_name = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_20_0: read-var effective_nse_ns:u8
        let s_20_0: u8 = fn_state.effective_nse_ns;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 2u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#4742 <= s_20_4
        fn_state.gs_4742 = s_20_4;
        // N s_20_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HaveRME(s_21_0)
        let s_21_1: bool = HaveRME(state, tracer, s_21_0);
        // D s_21_2: write-var gs#4741 <= s_21_1
        fn_state.gs_4741 = s_21_1;
        // N s_21_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var valid_name <= s_22_0
        fn_state.valid_name = s_22_0;
        // N s_22_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call SCR_GEN_read(s_23_0)
        let s_23_1: ProductType5c790c8ef59cc8b2 = SCR_GEN_read(state, tracer, s_23_0);
        // S s_23_2: call _get_SCRType_NS(s_23_1)
        let s_23_2: bool = u_get_SCRType_NS(state, tracer, s_23_1);
        // S s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #0u : u8
        let s_23_4: bool = false;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // S s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#4737 <= s_23_6
        fn_state.gs_4737 = s_23_6;
        // N s_23_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // D s_24_4: write-var gs#4736 <= s_24_3
        fn_state.gs_4736 = s_24_3;
        // N s_24_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_25_0: read-var el:u8
        let s_25_0: u8 = fn_state.el;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 2u16);
        // C s_25_2: const #432u : u32
        let s_25_2: u32 = 432;
        // D s_25_3: read-reg s_25_2:u8
        let s_25_3: u8 = {
            let value = state.read_register::<u8>(s_25_2 as isize);
            tracer.read_register(s_25_2 as isize, value);
            value
        };
        // D s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 2u16);
        // D s_25_5: cmp-eq s_25_1 s_25_4
        let s_25_5: bool = ((s_25_1) == (s_25_4));
        // D s_25_6: write-var gs#4735 <= s_25_5
        fn_state.gs_4735 = s_25_5;
        // N s_25_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_26_0: read-var mode:u8
        let s_26_0: u8 = fn_state.mode;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 5u16);
        // C s_26_2: const #400u : u32
        let s_26_2: u32 = 400;
        // D s_26_3: read-reg s_26_2:u8
        let s_26_3: u8 = {
            let value = state.read_register::<u8>(s_26_2 as isize);
            tracer.read_register(s_26_2 as isize, value);
            value
        };
        // D s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 5u16);
        // D s_26_5: cmp-eq s_26_1 s_26_4
        let s_26_5: bool = ((s_26_1) == (s_26_4));
        // D s_26_6: not s_26_5
        let s_26_6: bool = !s_26_5;
        // N s_26_7: branch s_26_6 b28 b27
        if s_26_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_27_0: const #432u : u32
        let s_27_0: u32 = 432;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: write-var el <= s_27_1
        fn_state.el = s_27_1;
        // N s_27_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_28_0: read-var mode:u8
        let s_28_0: u8 = fn_state.mode;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 5u16);
        // C s_28_2: const #360u : u32
        let s_28_2: u32 = 360;
        // D s_28_3: read-reg s_28_2:u8
        let s_28_3: u8 = {
            let value = state.read_register::<u8>(s_28_2 as isize);
            tracer.read_register(s_28_2 as isize, value);
            value
        };
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 5u16);
        // D s_28_5: cmp-eq s_28_1 s_28_4
        let s_28_5: bool = ((s_28_1) == (s_28_4));
        // D s_28_6: not s_28_5
        let s_28_6: bool = !s_28_5;
        // N s_28_7: branch s_28_6 b39 b29
        if s_28_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_29_0: const #424u : u32
        let s_29_0: u32 = 424;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // C s_29_2: const #2u : u8
        let s_29_2: u8 = 2;
        // D s_29_3: cmp-lt s_29_1 s_29_2
        let s_29_3: bool = ((s_29_1) < (s_29_2));
        // N s_29_4: branch s_29_3 b38 b30
        if s_29_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#4715 <= s_30_0
        fn_state.gs_4715 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_31_0: read-var gs#4715:u8
        let s_31_0: bool = fn_state.gs_4715;
        // N s_31_1: branch s_31_0 b37 b32
        if s_31_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#4716 <= s_32_0
        fn_state.gs_4716 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_33_0: read-var gs#4716:u8
        let s_33_0: bool = fn_state.gs_4716;
        // N s_33_1: branch s_33_0 b36 b34
        if s_33_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_34_0: const #440u : u32
        let s_34_0: u32 = 440;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: write-var el <= s_34_1
        fn_state.el = s_34_1;
        // N s_34_3: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_35_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: write-var el <= s_36_1
        fn_state.el = s_36_1;
        // N s_36_3: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_37_0: const #20920u : u32
        let s_37_0: u32 = 20920;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_SCR_Type_NS(s_37_1)
        let s_37_2: bool = u_get_SCR_Type_NS(state, tracer, s_37_1);
        // D s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // C s_37_4: const #0u : u8
        let s_37_4: bool = false;
        // C s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 1u16);
        // D s_37_6: cmp-eq s_37_3 s_37_5
        let s_37_6: bool = ((s_37_3) == (s_37_5));
        // D s_37_7: write-var gs#4716 <= s_37_6
        fn_state.gs_4716 = s_37_6;
        // N s_37_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HaveAArch64(s_38_0)
        let s_38_1: bool = HaveAArch64(state, tracer, s_38_0);
        // S s_38_2: not s_38_1
        let s_38_2: bool = !s_38_1;
        // D s_38_3: write-var gs#4715 <= s_38_2
        fn_state.gs_4715 = s_38_2;
        // N s_38_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_39_0: read-var mode:u8
        let s_39_0: u8 = fn_state.mode;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 5u16);
        // C s_39_2: const #368u : u32
        let s_39_2: u32 = 368;
        // D s_39_3: read-reg s_39_2:u8
        let s_39_3: u8 = {
            let value = state.read_register::<u8>(s_39_2 as isize);
            tracer.read_register(s_39_2 as isize, value);
            value
        };
        // D s_39_4: cast zx s_39_3 -> bv
        let s_39_4: Bits = Bits::new(s_39_3 as u128, 5u16);
        // D s_39_5: cmp-eq s_39_1 s_39_4
        let s_39_5: bool = ((s_39_1) == (s_39_4));
        // D s_39_6: not s_39_5
        let s_39_6: bool = !s_39_5;
        // N s_39_7: branch s_39_6 b50 b40
        if s_39_6 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_40_0: const #424u : u32
        let s_40_0: u32 = 424;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // C s_40_2: const #2u : u8
        let s_40_2: u8 = 2;
        // D s_40_3: cmp-lt s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) < (s_40_2));
        // N s_40_4: branch s_40_3 b49 b41
        if s_40_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#4718 <= s_41_0
        fn_state.gs_4718 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_42_0: read-var gs#4718:u8
        let s_42_0: bool = fn_state.gs_4718;
        // N s_42_1: branch s_42_0 b48 b43
        if s_42_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#4719 <= s_43_0
        fn_state.gs_4719 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_44_0: read-var gs#4719:u8
        let s_44_0: bool = fn_state.gs_4719;
        // N s_44_1: branch s_44_0 b47 b45
        if s_44_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_45_0: const #440u : u32
        let s_45_0: u32 = 440;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: write-var el <= s_45_1
        fn_state.el = s_45_1;
        // N s_45_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_46_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: write-var el <= s_47_1
        fn_state.el = s_47_1;
        // N s_47_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_48_0: const #20920u : u32
        let s_48_0: u32 = 20920;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_SCR_Type_NS(s_48_1)
        let s_48_2: bool = u_get_SCR_Type_NS(state, tracer, s_48_1);
        // D s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // C s_48_4: const #0u : u8
        let s_48_4: bool = false;
        // C s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 1u16);
        // D s_48_6: cmp-eq s_48_3 s_48_5
        let s_48_6: bool = ((s_48_3) == (s_48_5));
        // D s_48_7: write-var gs#4719 <= s_48_6
        fn_state.gs_4719 = s_48_6;
        // N s_48_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call HaveAArch64(s_49_0)
        let s_49_1: bool = HaveAArch64(state, tracer, s_49_0);
        // S s_49_2: not s_49_1
        let s_49_2: bool = !s_49_1;
        // D s_49_3: write-var gs#4718 <= s_49_2
        fn_state.gs_4718 = s_49_2;
        // N s_49_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_50_0: read-var mode:u8
        let s_50_0: u8 = fn_state.mode;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 5u16);
        // C s_50_2: const #376u : u32
        let s_50_2: u32 = 376;
        // D s_50_3: read-reg s_50_2:u8
        let s_50_3: u8 = {
            let value = state.read_register::<u8>(s_50_2 as isize);
            tracer.read_register(s_50_2 as isize, value);
            value
        };
        // D s_50_4: cast zx s_50_3 -> bv
        let s_50_4: Bits = Bits::new(s_50_3 as u128, 5u16);
        // D s_50_5: cmp-eq s_50_1 s_50_4
        let s_50_5: bool = ((s_50_1) == (s_50_4));
        // D s_50_6: not s_50_5
        let s_50_6: bool = !s_50_5;
        // N s_50_7: branch s_50_6 b61 b51
        if s_50_6 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_51_0: const #424u : u32
        let s_51_0: u32 = 424;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // C s_51_2: const #2u : u8
        let s_51_2: u8 = 2;
        // D s_51_3: cmp-lt s_51_1 s_51_2
        let s_51_3: bool = ((s_51_1) < (s_51_2));
        // N s_51_4: branch s_51_3 b60 b52
        if s_51_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#4721 <= s_52_0
        fn_state.gs_4721 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_53_0: read-var gs#4721:u8
        let s_53_0: bool = fn_state.gs_4721;
        // N s_53_1: branch s_53_0 b59 b54
        if s_53_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#4722 <= s_54_0
        fn_state.gs_4722 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_55_0: read-var gs#4722:u8
        let s_55_0: bool = fn_state.gs_4722;
        // N s_55_1: branch s_55_0 b58 b56
        if s_55_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_56_0: const #440u : u32
        let s_56_0: u32 = 440;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: write-var el <= s_56_1
        fn_state.el = s_56_1;
        // N s_56_3: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_57_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: write-var el <= s_58_1
        fn_state.el = s_58_1;
        // N s_58_3: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_59_0: const #20920u : u32
        let s_59_0: u32 = 20920;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_SCR_Type_NS(s_59_1)
        let s_59_2: bool = u_get_SCR_Type_NS(state, tracer, s_59_1);
        // D s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // C s_59_4: const #0u : u8
        let s_59_4: bool = false;
        // C s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 1u16);
        // D s_59_6: cmp-eq s_59_3 s_59_5
        let s_59_6: bool = ((s_59_3) == (s_59_5));
        // D s_59_7: write-var gs#4722 <= s_59_6
        fn_state.gs_4722 = s_59_6;
        // N s_59_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call HaveAArch64(s_60_0)
        let s_60_1: bool = HaveAArch64(state, tracer, s_60_0);
        // S s_60_2: not s_60_1
        let s_60_2: bool = !s_60_1;
        // D s_60_3: write-var gs#4721 <= s_60_2
        fn_state.gs_4721 = s_60_2;
        // N s_60_4: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_61_0: read-var mode:u8
        let s_61_0: u8 = fn_state.mode;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 5u16);
        // C s_61_2: const #392u : u32
        let s_61_2: u32 = 392;
        // D s_61_3: read-reg s_61_2:u8
        let s_61_3: u8 = {
            let value = state.read_register::<u8>(s_61_2 as isize);
            tracer.read_register(s_61_2 as isize, value);
            value
        };
        // D s_61_4: cast zx s_61_3 -> bv
        let s_61_4: Bits = Bits::new(s_61_3 as u128, 5u16);
        // D s_61_5: cmp-eq s_61_1 s_61_4
        let s_61_5: bool = ((s_61_1) == (s_61_4));
        // D s_61_6: not s_61_5
        let s_61_6: bool = !s_61_5;
        // N s_61_7: branch s_61_6 b72 b62
        if s_61_6 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // N s_62_4: branch s_62_3 b71 b63
        if s_62_3 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#4724 <= s_63_0
        fn_state.gs_4724 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_64_0: read-var gs#4724:u8
        let s_64_0: bool = fn_state.gs_4724;
        // N s_64_1: branch s_64_0 b70 b65
        if s_64_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#4725 <= s_65_0
        fn_state.gs_4725 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_66_0: read-var gs#4725:u8
        let s_66_0: bool = fn_state.gs_4725;
        // N s_66_1: branch s_66_0 b69 b67
        if s_66_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_67_0: const #440u : u32
        let s_67_0: u32 = 440;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: write-var el <= s_67_1
        fn_state.el = s_67_1;
        // N s_67_3: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_68_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_69_0: const #424u : u32
        let s_69_0: u32 = 424;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: write-var el <= s_69_1
        fn_state.el = s_69_1;
        // N s_69_3: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_70_0: const #20920u : u32
        let s_70_0: u32 = 20920;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_SCR_Type_NS(s_70_1)
        let s_70_2: bool = u_get_SCR_Type_NS(state, tracer, s_70_1);
        // D s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // C s_70_4: const #0u : u8
        let s_70_4: bool = false;
        // C s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 1u16);
        // D s_70_6: cmp-eq s_70_3 s_70_5
        let s_70_6: bool = ((s_70_3) == (s_70_5));
        // D s_70_7: write-var gs#4725 <= s_70_6
        fn_state.gs_4725 = s_70_6;
        // N s_70_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call HaveAArch64(s_71_0)
        let s_71_1: bool = HaveAArch64(state, tracer, s_71_0);
        // S s_71_2: not s_71_1
        let s_71_2: bool = !s_71_1;
        // D s_71_3: write-var gs#4724 <= s_71_2
        fn_state.gs_4724 = s_71_2;
        // N s_71_4: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_72_0: read-var mode:u8
        let s_72_0: u8 = fn_state.mode;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 5u16);
        // C s_72_2: const #408u : u32
        let s_72_2: u32 = 408;
        // D s_72_3: read-reg s_72_2:u8
        let s_72_3: u8 = {
            let value = state.read_register::<u8>(s_72_2 as isize);
            tracer.read_register(s_72_2 as isize, value);
            value
        };
        // D s_72_4: cast zx s_72_3 -> bv
        let s_72_4: Bits = Bits::new(s_72_3 as u128, 5u16);
        // D s_72_5: cmp-eq s_72_1 s_72_4
        let s_72_5: bool = ((s_72_1) == (s_72_4));
        // D s_72_6: not s_72_5
        let s_72_6: bool = !s_72_5;
        // N s_72_7: branch s_72_6 b83 b73
        if s_72_6 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_73_0: const #424u : u32
        let s_73_0: u32 = 424;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // C s_73_2: const #2u : u8
        let s_73_2: u8 = 2;
        // D s_73_3: cmp-lt s_73_1 s_73_2
        let s_73_3: bool = ((s_73_1) < (s_73_2));
        // N s_73_4: branch s_73_3 b82 b74
        if s_73_3 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#4727 <= s_74_0
        fn_state.gs_4727 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_75_0: read-var gs#4727:u8
        let s_75_0: bool = fn_state.gs_4727;
        // N s_75_1: branch s_75_0 b81 b76
        if s_75_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#4728 <= s_76_0
        fn_state.gs_4728 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_77_0: read-var gs#4728:u8
        let s_77_0: bool = fn_state.gs_4728;
        // N s_77_1: branch s_77_0 b80 b78
        if s_77_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_78_0: const #440u : u32
        let s_78_0: u32 = 440;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: write-var el <= s_78_1
        fn_state.el = s_78_1;
        // N s_78_3: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_79_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_80_0: const #424u : u32
        let s_80_0: u32 = 424;
        // D s_80_1: read-reg s_80_0:u8
        let s_80_1: u8 = {
            let value = state.read_register::<u8>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: write-var el <= s_80_1
        fn_state.el = s_80_1;
        // N s_80_3: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_81_0: const #20920u : u32
        let s_81_0: u32 = 20920;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_SCR_Type_NS(s_81_1)
        let s_81_2: bool = u_get_SCR_Type_NS(state, tracer, s_81_1);
        // D s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // C s_81_4: const #0u : u8
        let s_81_4: bool = false;
        // C s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 1u16);
        // D s_81_6: cmp-eq s_81_3 s_81_5
        let s_81_6: bool = ((s_81_3) == (s_81_5));
        // D s_81_7: write-var gs#4728 <= s_81_6
        fn_state.gs_4728 = s_81_6;
        // N s_81_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call HaveAArch64(s_82_0)
        let s_82_1: bool = HaveAArch64(state, tracer, s_82_0);
        // S s_82_2: not s_82_1
        let s_82_2: bool = !s_82_1;
        // D s_82_3: write-var gs#4727 <= s_82_2
        fn_state.gs_4727 = s_82_2;
        // N s_82_4: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_83_0: read-var mode:u8
        let s_83_0: u8 = fn_state.mode;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 5u16);
        // C s_83_2: const #416u : u32
        let s_83_2: u32 = 416;
        // D s_83_3: read-reg s_83_2:u8
        let s_83_3: u8 = {
            let value = state.read_register::<u8>(s_83_2 as isize);
            tracer.read_register(s_83_2 as isize, value);
            value
        };
        // D s_83_4: cast zx s_83_3 -> bv
        let s_83_4: Bits = Bits::new(s_83_3 as u128, 5u16);
        // D s_83_5: cmp-eq s_83_1 s_83_4
        let s_83_5: bool = ((s_83_1) == (s_83_4));
        // D s_83_6: not s_83_5
        let s_83_6: bool = !s_83_5;
        // N s_83_7: branch s_83_6 b94 b84
        if s_83_6 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_84_0: const #424u : u32
        let s_84_0: u32 = 424;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // C s_84_2: const #2u : u8
        let s_84_2: u8 = 2;
        // D s_84_3: cmp-lt s_84_1 s_84_2
        let s_84_3: bool = ((s_84_1) < (s_84_2));
        // N s_84_4: branch s_84_3 b93 b85
        if s_84_3 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#4730 <= s_85_0
        fn_state.gs_4730 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_86_0: read-var gs#4730:u8
        let s_86_0: bool = fn_state.gs_4730;
        // N s_86_1: branch s_86_0 b92 b87
        if s_86_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#4731 <= s_87_0
        fn_state.gs_4731 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_88_0: read-var gs#4731:u8
        let s_88_0: bool = fn_state.gs_4731;
        // N s_88_1: branch s_88_0 b91 b89
        if s_88_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_89_0: const #440u : u32
        let s_89_0: u32 = 440;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: write-var el <= s_89_1
        fn_state.el = s_89_1;
        // N s_89_3: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_90_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_91_0: const #424u : u32
        let s_91_0: u32 = 424;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: write-var el <= s_91_1
        fn_state.el = s_91_1;
        // N s_91_3: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_92_0: const #20920u : u32
        let s_92_0: u32 = 20920;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call _get_SCR_Type_NS(s_92_1)
        let s_92_2: bool = u_get_SCR_Type_NS(state, tracer, s_92_1);
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // C s_92_4: const #0u : u8
        let s_92_4: bool = false;
        // C s_92_5: cast zx s_92_4 -> bv
        let s_92_5: Bits = Bits::new(s_92_4 as u128, 1u16);
        // D s_92_6: cmp-eq s_92_3 s_92_5
        let s_92_6: bool = ((s_92_3) == (s_92_5));
        // D s_92_7: write-var gs#4731 <= s_92_6
        fn_state.gs_4731 = s_92_6;
        // N s_92_8: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call HaveAArch64(s_93_0)
        let s_93_1: bool = HaveAArch64(state, tracer, s_93_0);
        // S s_93_2: not s_93_1
        let s_93_2: bool = !s_93_1;
        // D s_93_3: write-var gs#4730 <= s_93_2
        fn_state.gs_4730 = s_93_2;
        // N s_93_4: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_94_0: read-var mode:u8
        let s_94_0: u8 = fn_state.mode;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 5u16);
        // C s_94_2: const #352u : u32
        let s_94_2: u32 = 352;
        // D s_94_3: read-reg s_94_2:u8
        let s_94_3: u8 = {
            let value = state.read_register::<u8>(s_94_2 as isize);
            tracer.read_register(s_94_2 as isize, value);
            value
        };
        // D s_94_4: cast zx s_94_3 -> bv
        let s_94_4: Bits = Bits::new(s_94_3 as u128, 5u16);
        // D s_94_5: cmp-eq s_94_1 s_94_4
        let s_94_5: bool = ((s_94_1) == (s_94_4));
        // D s_94_6: not s_94_5
        let s_94_6: bool = !s_94_5;
        // N s_94_7: branch s_94_6 b96 b95
        if s_94_6 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_95_0: const #448u : u32
        let s_95_0: u32 = 448;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: write-var el <= s_95_1
        fn_state.el = s_95_1;
        // N s_95_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var valid_name <= s_96_0
        fn_state.valid_name = s_96_0;
        // N s_96_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
