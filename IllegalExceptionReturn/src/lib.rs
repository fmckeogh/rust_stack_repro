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
use ELFromSPSR::*;
use Halted::*;
use ELUsingAArch32::*;
use GetCurrentEXLOCKEN::*;
use ELUsingAArch32K::*;
use UsingAArch32::*;
use IsSecureBelowEL3::*;
use IsSecureEL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use HaveGCS::*;
use common::*;
pub fn IllegalExceptionReturn<T: Tracer>(
    state: &mut State,
    tracer: &T,
    spsr: Bits,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        target: u8,
        known: bool,
        gs_4821: bool,
        gs_4825: bool,
        return_value: bool,
        gs_4822: bool,
        gs_4830: bool,
        ga_3201: ProductTypea5cc8de4daab131c,
        gs_4824: bool,
        gs_4827: bool,
        gs_4826: bool,
        gs_4823: bool,
        spsr_mode_is_aarch32: bool,
        gs_4818: bool,
        gs_4819: bool,
        ga_3207: ProductType8b847afc727d5818,
        gs_4820: bool,
        target_el_is_aarch32: bool,
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
    ) -> bool {
        // D s_0_0: read-var spsr:bv
        let s_0_0: Bits = fn_state.spsr;
        // D s_0_1: call ELFromSPSR(s_0_0)
        let s_0_1: ProductTypea5cc8de4daab131c = ELFromSPSR(state, tracer, s_0_0);
        // D s_0_2: write-var ga#3201 <= s_0_1
        fn_state.ga_3201 = s_0_1;
        // D s_0_3: read-var ga#3201.0:struct
        let s_0_3: bool = fn_state.ga_3201._0;
        // D s_0_4: read-var ga#3201.1:struct
        let s_0_4: u8 = fn_state.ga_3201._1;
        // D s_0_5: write-var target <= s_0_4
        fn_state.target = s_0_4;
        // D s_0_6: not s_0_3
        let s_0_6: bool = !s_0_3;
        // N s_0_7: branch s_0_6 b49 b1
        if s_0_6 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var target:u8
        let s_1_0: u8 = fn_state.target;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #16975u : u32
        let s_1_4: u32 = 16975;
        // D s_1_5: read-reg s_1_4:u8
        let s_1_5: u8 = {
            let value = state.read_register::<u8>(s_1_4 as isize);
            tracer.read_register(s_1_4 as isize, value);
            value
        };
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 2u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: cast zx s_1_3 -> i
        let s_1_9: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_10: cast zx s_1_8 -> i
        let s_1_10: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_11: cmp-gt s_1_9 s_1_10
        let s_1_11: bool = ((s_1_9) > (s_1_10));
        // N s_1_12: branch s_1_11 b48 b2
        if s_1_11 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var spsr:bv
        let s_2_1: Bits = fn_state.spsr;
        // C s_2_2: const #1u : u64
        let s_2_2: u64 = 1;
        // D s_2_3: bit-extract s_2_1 s_2_0 s_2_2
        let s_2_3: Bits = (Bits::new(
            ((s_2_1) >> (s_2_0)).value(),
            u16::try_from(s_2_2).unwrap(),
        ));
        // D s_2_4: cast reint s_2_3 -> u8
        let s_2_4: bool = ((s_2_3.value()) != 0);
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // C s_2_6: const #0u : u64
        let s_2_6: u64 = 0;
        // D s_2_7: cast zx s_2_4 -> u64
        let s_2_7: u64 = (s_2_4 as u64);
        // C s_2_8: const #1u : u64
        let s_2_8: u64 = 1;
        // D s_2_9: and s_2_7 s_2_8
        let s_2_9: u64 = ((s_2_7) & (s_2_8));
        // D s_2_10: cmp-eq s_2_9 s_2_8
        let s_2_10: bool = ((s_2_9) == (s_2_8));
        // D s_2_11: lsl s_2_7 s_2_5
        let s_2_11: u64 = s_2_7 << s_2_5;
        // D s_2_12: or s_2_6 s_2_11
        let s_2_12: u64 = ((s_2_6) | (s_2_11));
        // D s_2_13: cmpl s_2_11
        let s_2_13: u64 = !s_2_11;
        // D s_2_14: and s_2_6 s_2_13
        let s_2_14: u64 = ((s_2_6) & (s_2_13));
        // D s_2_15: select s_2_10 s_2_12 s_2_14
        let s_2_15: u64 = if s_2_10 { s_2_12 } else { s_2_14 };
        // D s_2_16: cast trunc s_2_15 -> u8
        let s_2_16: bool = ((s_2_15) != 0);
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // C s_2_18: const #1u : u8
        let s_2_18: bool = true;
        // C s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // D s_2_20: cmp-eq s_2_17 s_2_19
        let s_2_20: bool = ((s_2_17) == (s_2_19));
        // D s_2_21: write-var spsr_mode_is_aarch32 <= s_2_20
        fn_state.spsr_mode_is_aarch32 = s_2_20;
        // D s_2_22: read-var target:u8
        let s_2_22: u8 = fn_state.target;
        // D s_2_23: call ELUsingAArch32K(s_2_22)
        let s_2_23: ProductType8b847afc727d5818 = ELUsingAArch32K(state, tracer, s_2_22);
        // D s_2_24: write-var ga#3207 <= s_2_23
        fn_state.ga_3207 = s_2_23;
        // D s_2_25: read-var ga#3207.0:struct
        let s_2_25: bool = fn_state.ga_3207._0;
        // D s_2_26: read-var ga#3207.1:struct
        let s_2_26: bool = fn_state.ga_3207._1;
        // D s_2_27: write-var known <= s_2_25
        fn_state.known = s_2_25;
        // D s_2_28: write-var target_el_is_aarch32 <= s_2_26
        fn_state.target_el_is_aarch32 = s_2_26;
        // D s_2_29: read-var known:u8
        let s_2_29: bool = fn_state.known;
        // N s_2_30: branch s_2_29 b47 b3
        if s_2_29 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var target:u8
        let s_3_0: u8 = fn_state.target;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #448u : u32
        let s_3_2: u32 = 448;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b46 b4
        if s_3_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#4818 <= s_4_0
        fn_state.gs_4818 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#4818:u8
        let s_5_0: bool = fn_state.gs_4818;
        // D s_5_1: write-var gs#4819 <= s_5_0
        fn_state.gs_4819 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#4819:u8
        let s_6_0: bool = fn_state.gs_4819;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var known:u8
        let s_6_2: bool = fn_state.known;
        // N s_6_3: branch s_6_2 b45 b7
        if s_6_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#4820 <= s_7_0
        fn_state.gs_4820 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#4820:u8
        let s_8_0: bool = fn_state.gs_4820;
        // N s_8_1: branch s_8_0 b44 b9
        if s_8_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call UsingAArch32(s_9_0)
        let s_9_1: bool = UsingAArch32(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b43 b10
        if s_9_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#4821 <= s_10_0
        fn_state.gs_4821 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#4821:u8
        let s_11_0: bool = fn_state.gs_4821;
        // N s_11_1: branch s_11_0 b42 b12
        if s_11_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #432u : u32
        let s_12_0: u32 = 432;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // D s_12_3: cmp-lt s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) < (s_12_2));
        // N s_12_4: branch s_12_3 b41 b13
        if s_12_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#4822 <= s_13_0
        fn_state.gs_4822 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#4822:u8
        let s_14_0: bool = fn_state.gs_4822;
        // N s_14_1: branch s_14_0 b40 b15
        if s_14_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#4823 <= s_15_0
        fn_state.gs_4823 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#4823:u8
        let s_16_0: bool = fn_state.gs_4823;
        // N s_16_1: branch s_16_0 b34 b17
        if s_16_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveGCS(s_18_0)
        let s_18_1: bool = HaveGCS(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b33 b19
        if s_18_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#4824 <= s_19_0
        fn_state.gs_4824 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#4824:u8
        let s_20_0: bool = fn_state.gs_4824;
        // N s_20_1: branch s_20_0 b32 b21
        if s_20_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#4825 <= s_21_0
        fn_state.gs_4825 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#4825:u8
        let s_22_0: bool = fn_state.gs_4825;
        // N s_22_1: branch s_22_0 b31 b23
        if s_22_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#4826 <= s_23_0
        fn_state.gs_4826 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#4826:u8
        let s_24_0: bool = fn_state.gs_4826;
        // N s_24_1: branch s_24_0 b30 b25
        if s_24_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#4827 <= s_25_0
        fn_state.gs_4827 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#4827:u8
        let s_26_0: bool = fn_state.gs_4827;
        // N s_26_1: branch s_26_0 b29 b27
        if s_26_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var return_value <= s_27_0
        fn_state.return_value = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var return_value:u8
        let s_28_0: bool = fn_state.return_value;
        // N s_28_1: return s_28_0
        return s_28_0;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var return_value <= s_29_0
        fn_state.return_value = s_29_0;
        // N s_29_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call Halted(s_30_0)
        let s_30_1: bool = Halted(state, tracer, s_30_0);
        // S s_30_2: not s_30_1
        let s_30_2: bool = !s_30_1;
        // D s_30_3: write-var gs#4827 <= s_30_2
        fn_state.gs_4827 = s_30_2;
        // N s_30_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call GetCurrentEXLOCKEN(s_31_0)
        let s_31_1: bool = GetCurrentEXLOCKEN(state, tracer, s_31_0);
        // D s_31_2: write-var gs#4826 <= s_31_1
        fn_state.gs_4826 = s_31_1;
        // N s_31_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #16975u : u32
        let s_32_0: u32 = 16975;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 2u16);
        // D s_32_3: read-var target:u8
        let s_32_3: u8 = fn_state.target;
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 2u16);
        // D s_32_5: cmp-eq s_32_2 s_32_4
        let s_32_5: bool = ((s_32_2) == (s_32_4));
        // D s_32_6: write-var gs#4825 <= s_32_5
        fn_state.gs_4825 = s_32_5;
        // N s_32_7: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #16976u : u32
        let s_33_0: u32 = 16976;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: bool = {
            let value = state.read_register::<bool>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: cast zx s_33_1 -> bv
        let s_33_2: Bits = Bits::new(s_33_1 as u128, 1u16);
        // C s_33_3: const #0u : u8
        let s_33_3: bool = false;
        // C s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 1u16);
        // D s_33_5: cmp-eq s_33_2 s_33_4
        let s_33_5: bool = ((s_33_2) == (s_33_4));
        // D s_33_6: write-var gs#4824 <= s_33_5
        fn_state.gs_4824 = s_33_5;
        // N s_33_7: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call IsSecureBelowEL3(s_34_0)
        let s_34_1: bool = IsSecureBelowEL3(state, tracer, s_34_0);
        // S s_34_2: not s_34_1
        let s_34_2: bool = !s_34_1;
        // N s_34_3: branch s_34_2 b39 b35
        if s_34_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call IsSecureEL2Enabled(s_35_0)
        let s_35_1: bool = IsSecureEL2Enabled(state, tracer, s_35_0);
        // D s_35_2: write-var gs#4830 <= s_35_1
        fn_state.gs_4830 = s_35_1;
        // N s_35_3: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var gs#4830:u8
        let s_36_0: bool = fn_state.gs_4830;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_37_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var return_value <= s_38_0
        fn_state.return_value = s_38_0;
        // N s_38_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#4830 <= s_39_0
        fn_state.gs_4830 = s_39_0;
        // N s_39_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #102552u : u32
        let s_40_0: u32 = 102552;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call _get_HCR_EL2_Type_TGE(s_40_1)
        let s_40_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_40_1);
        // D s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // C s_40_4: const #1u : u8
        let s_40_4: bool = true;
        // C s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 1u16);
        // D s_40_6: cmp-eq s_40_3 s_40_5
        let s_40_6: bool = ((s_40_3) == (s_40_5));
        // D s_40_7: write-var gs#4823 <= s_40_6
        fn_state.gs_4823 = s_40_6;
        // N s_40_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var target:u8
        let s_41_0: u8 = fn_state.target;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 2u16);
        // C s_41_2: const #440u : u32
        let s_41_2: u32 = 440;
        // D s_41_3: read-reg s_41_2:u8
        let s_41_3: u8 = {
            let value = state.read_register::<u8>(s_41_2 as isize);
            tracer.read_register(s_41_2 as isize, value);
            value
        };
        // D s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 2u16);
        // D s_41_5: cmp-eq s_41_1 s_41_4
        let s_41_5: bool = ((s_41_1) == (s_41_4));
        // D s_41_6: write-var gs#4822 <= s_41_5
        fn_state.gs_4822 = s_41_5;
        // N s_41_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var return_value <= s_42_0
        fn_state.return_value = s_42_0;
        // N s_42_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var spsr_mode_is_aarch32:u8
        let s_43_0: bool = fn_state.spsr_mode_is_aarch32;
        // D s_43_1: not s_43_0
        let s_43_1: bool = !s_43_0;
        // D s_43_2: write-var gs#4821 <= s_43_1
        fn_state.gs_4821 = s_43_1;
        // N s_43_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var return_value <= s_44_0
        fn_state.return_value = s_44_0;
        // N s_44_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var spsr_mode_is_aarch32:u8
        let s_45_0: bool = fn_state.spsr_mode_is_aarch32;
        // D s_45_1: read-var target_el_is_aarch32:u8
        let s_45_1: bool = fn_state.target_el_is_aarch32;
        // D s_45_2: cmp-ne s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) != (s_45_1));
        // D s_45_3: write-var gs#4820 <= s_45_2
        fn_state.gs_4820 = s_45_2;
        // N s_45_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #440u : u32
        let s_46_0: u32 = 440;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call ELUsingAArch32(s_46_1)
        let s_46_2: bool = ELUsingAArch32(state, tracer, s_46_1);
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // D s_46_4: write-var gs#4818 <= s_46_3
        fn_state.gs_4818 = s_46_3;
        // N s_46_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#4819 <= s_47_0
        fn_state.gs_4819 = s_47_0;
        // N s_47_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var return_value <= s_48_0
        fn_state.return_value = s_48_0;
        // N s_48_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var return_value <= s_49_0
        fn_state.return_value = s_49_0;
        // N s_49_2: jump b28
        return block_28(state, tracer, fn_state);
    }
}
