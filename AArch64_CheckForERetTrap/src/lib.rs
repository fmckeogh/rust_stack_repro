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
use ThisInstrAddr::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use HaveNVExt::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use HaveFGTExt::*;
use u_get_HCR_EL2_Type_NV::*;
use Bit::*;
use u_get_HFGITR_EL2_Type_ERET::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_CheckForERetTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    eret_with_pac: bool,
    pac_uses_key_a: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_24721: bool,
        except: ProductTypeb7f99f96751e17c4,
        gs_24727: bool,
        gs_24724: bool,
        preferred_exception_return: u64,
        gs_24726: bool,
        gs_24723: bool,
        vect_offsetshadow_475: i64,
        gs_24722: bool,
        gs_24725: bool,
        eret_with_pac: bool,
        pac_uses_key_a: bool,
    }
    let fn_state = FunctionState {
        eret_with_pac,
        pac_uses_key_a,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #440u : u32
        let s_0_3: u32 = 440;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b28 b1
        if s_0_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#24721 <= s_1_0
        fn_state.gs_24721 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24721:u8
        let s_2_0: bool = fn_state.gs_24721;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#24727 <= s_3_0
        fn_state.gs_24727 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24727:u8
        let s_4_0: bool = fn_state.gs_24727;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
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
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // S s_6_2: call ThisInstrAddr(s_6_1)
        let s_6_2: Bits = ThisInstrAddr(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // D s_6_4: write-var preferred_exception_return <= s_6_3
        fn_state.preferred_exception_return = s_6_3;
        // C s_6_5: const #0u : u8
        let s_6_5: u8 = 0;
        // C s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 4u16);
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (s_6_6.value() as i128);
        // C s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: write-var vect_offsetshadow#475 <= s_6_8
        fn_state.vect_offsetshadow_475 = s_6_8;
        // C s_6_10: const #16u : u32
        let s_6_10: u32 = 16;
        // S s_6_11: call ExceptionSyndrome(s_6_10)
        let s_6_11: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_6_10,
        );
        // D s_6_12: write-var except <= s_6_11
        fn_state.except = s_6_11;
        // D s_6_13: read-var eret_with_pac:u8
        let s_6_13: bool = fn_state.eret_with_pac;
        // D s_6_14: not s_6_13
        let s_6_14: bool = !s_6_13;
        // N s_6_15: branch s_6_14 b11 b7
        if s_6_14 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // D s_7_2: read-var except:struct
        let s_7_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_3: write-var except <= s_7_2
        fn_state.except = s_7_2;
        // D s_7_4: read-var pac_uses_key_a:u8
        let s_7_4: bool = fn_state.pac_uses_key_a;
        // N s_7_5: branch s_7_4 b10 b8
        if s_7_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // S s_8_1: call Bit(s_8_0)
        let s_8_1: bool = Bit(state, tracer, s_8_0);
        // D s_8_2: read-var except:struct
        let s_8_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_3: write-var except <= s_8_2
        fn_state.except = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var vect_offsetshadow#475:i64
        let s_9_0: i64 = fn_state.vect_offsetshadow_475;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: read-var except:struct
        let s_9_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_9_5: read-var preferred_exception_return:u64
        let s_9_5: u64 = fn_state.preferred_exception_return;
        // D s_9_6: call AArch64_TakeException(s_9_3, s_9_4, s_9_5, s_9_1)
        let s_9_6: () = AArch64_TakeException(state, tracer, s_9_3, s_9_4, s_9_5, s_9_1);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // S s_10_1: call Bit(s_10_0)
        let s_10_1: bool = Bit(state, tracer, s_10_0);
        // D s_10_2: read-var except:struct
        let s_10_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_10_3: write-var except <= s_10_2
        fn_state.except = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // S s_11_1: call Bit(s_11_0)
        let s_11_1: bool = Bit(state, tracer, s_11_0);
        // D s_11_2: read-var except:struct
        let s_11_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_11_3: write-var except <= s_11_2
        fn_state.except = s_11_2;
        // C s_11_4: const #0u : u8
        let s_11_4: bool = false;
        // S s_11_5: call Bit(s_11_4)
        let s_11_5: bool = Bit(state, tracer, s_11_4);
        // D s_11_6: read-var except:struct
        let s_11_6: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_11_7: write-var except <= s_11_6
        fn_state.except = s_11_6;
        // N s_11_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HaveNVExt(s_12_0)
        let s_12_1: bool = HaveNVExt(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b27 b13
        if s_12_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#24722 <= s_13_0
        fn_state.gs_24722 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#24722:u8
        let s_14_0: bool = fn_state.gs_24722;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveFGTExt(s_15_0)
        let s_15_1: bool = HaveFGTExt(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b22 b16
        if s_15_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#24724 <= s_16_0
        fn_state.gs_24724 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#24724:u8
        let s_17_0: bool = fn_state.gs_24724;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#24725 <= s_18_0
        fn_state.gs_24725 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#24725:u8
        let s_19_0: bool = fn_state.gs_24725;
        // D s_19_1: write-var gs#24726 <= s_19_0
        fn_state.gs_24726 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#24726:u8
        let s_20_0: bool = fn_state.gs_24726;
        // D s_20_1: write-var gs#24727 <= s_20_0
        fn_state.gs_24727 = s_20_0;
        // N s_20_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #13608u : u32
        let s_21_0: u32 = 13608;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HFGITR_EL2_Type_ERET(s_21_1)
        let s_21_2: bool = u_get_HFGITR_EL2_Type_ERET(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#24725 <= s_21_6
        fn_state.gs_24725 = s_21_6;
        // N s_21_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // D s_22_4: not s_22_3
        let s_22_4: bool = !s_22_3;
        // N s_22_5: branch s_22_4 b25 b23
        if s_22_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #90704u : u32
        let s_23_0: u32 = 90704;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_SCR_EL3_Type_FGTEn(s_23_1)
        let s_23_2: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#24723 <= s_23_6
        fn_state.gs_24723 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#24723:u8
        let s_24_0: bool = fn_state.gs_24723;
        // D s_24_1: write-var gs#24724 <= s_24_0
        fn_state.gs_24724 = s_24_0;
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#24723 <= s_25_0
        fn_state.gs_24723 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#24726 <= s_26_0
        fn_state.gs_24726 = s_26_0;
        // N s_26_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #102552u : u32
        let s_27_0: u32 = 102552;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_HCR_EL2_Type_NV(s_27_1)
        let s_27_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #1u : u8
        let s_27_4: bool = true;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#24722 <= s_27_6
        fn_state.gs_24722 = s_27_6;
        // N s_27_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // D s_28_2: write-var gs#24721 <= s_28_1
        fn_state.gs_24721 = s_28_1;
        // N s_28_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
