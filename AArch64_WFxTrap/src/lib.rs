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
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use Bit::*;
use ConditionSyndrome::*;
use ThisInstr::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_WFxTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    wfxtype: u32,
    target_el: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6608: bool,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        gs_6607: bool,
        vect_offset: i64,
        wfxtype: u32,
        target_el: u8,
    }
    let fn_state = FunctionState {
        wfxtype,
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var target_el:u8
        let s_0_0: u8 = fn_state.target_el;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #16975u : u32
        let s_0_4: u32 = 16975;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: cast zx s_0_3 -> i
        let s_0_9: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_10: cast zx s_0_8 -> i
        let s_0_10: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_11: cmp-gt s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) > (s_0_10));
        // N s_0_12: assert s_0_11
        let s_0_12: () = assert!(s_0_11);
        // C s_0_13: const #64s : i64
        let s_0_13: i64 = 64;
        // C s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // S s_0_15: call ThisInstrAddr(s_0_14)
        let s_0_15: Bits = ThisInstrAddr(state, tracer, s_0_14);
        // S s_0_16: cast reint s_0_15 -> u64
        let s_0_16: u64 = (s_0_15.value() as u64);
        // D s_0_17: write-var preferred_exception_return <= s_0_16
        fn_state.preferred_exception_return = s_0_16;
        // C s_0_18: const #0u : u8
        let s_0_18: u8 = 0;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 4u16);
        // C s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (s_0_19.value() as i128);
        // C s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: write-var vect_offset <= s_0_21
        fn_state.vect_offset = s_0_21;
        // C s_0_23: const #1u : u32
        let s_0_23: u32 = 1;
        // S s_0_24: call ExceptionSyndrome(s_0_23)
        let s_0_24: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_0_23,
        );
        // D s_0_25: write-var except <= s_0_24
        fn_state.except = s_0_24;
        // C s_0_26: const #() : ()
        let s_0_26: () = ();
        // S s_0_27: call ConditionSyndrome(s_0_26)
        let s_0_27: u8 = ConditionSyndrome(state, tracer, s_0_26);
        // D s_0_28: read-var except:struct
        let s_0_28: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_0_29: write-var except <= s_0_28
        fn_state.except = s_0_28;
        // C s_0_30: const #1u : u32
        let s_0_30: u32 = 1;
        // D s_0_31: read-var wfxtype:u32
        let s_0_31: u32 = fn_state.wfxtype;
        // D s_0_32: cmp-eq s_0_30 s_0_31
        let s_0_32: bool = ((s_0_30) == (s_0_31));
        // D s_0_33: not s_0_32
        let s_0_33: bool = !s_0_32;
        // N s_0_34: branch s_0_33 b11 b1
        if s_0_33 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var except:struct
        let s_1_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_1_1: write-var except <= s_1_0
        fn_state.except = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var target_el:u8
        let s_2_0: u8 = fn_state.target_el;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#6607 <= s_3_0
        fn_state.gs_6607 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#6607:u8
        let s_4_0: bool = fn_state.gs_6607;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#6608 <= s_5_0
        fn_state.gs_6608 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#6608:u8
        let s_6_0: bool = fn_state.gs_6608;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
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
        // D s_7_0: read-var vect_offset:i64
        let s_7_0: i64 = fn_state.vect_offset;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var target_el:u8
        let s_7_2: u8 = fn_state.target_el;
        // D s_7_3: read-var except:struct
        let s_7_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_7_4: read-var preferred_exception_return:u64
        let s_7_4: u64 = fn_state.preferred_exception_return;
        // D s_7_5: call AArch64_TakeException(s_7_2, s_7_3, s_7_4, s_7_1)
        let s_7_5: () = AArch64_TakeException(state, tracer, s_7_2, s_7_3, s_7_4, s_7_1);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var vect_offset:i64
        let s_8_0: i64 = fn_state.vect_offset;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // C s_8_2: const #432u : u32
        let s_8_2: u32 = 432;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: read-var except:struct
        let s_8_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_5: read-var preferred_exception_return:u64
        let s_8_5: u64 = fn_state.preferred_exception_return;
        // D s_8_6: call AArch64_TakeException(s_8_3, s_8_4, s_8_5, s_8_1)
        let s_8_6: () = AArch64_TakeException(state, tracer, s_8_3, s_8_4, s_8_5, s_8_1);
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #102552u : u32
        let s_9_0: u32 = 102552;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_HCR_EL2_Type_TGE(s_9_1)
        let s_9_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#6608 <= s_9_6
        fn_state.gs_6608 = s_9_6;
        // N s_9_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EL2Enabled(s_10_0)
        let s_10_1: bool = EL2Enabled(state, tracer, s_10_0);
        // D s_10_2: write-var gs#6607 <= s_10_1
        fn_state.gs_6607 = s_10_1;
        // N s_10_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: read-var wfxtype:u32
        let s_11_1: u32 = fn_state.wfxtype;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var except:struct
        let s_12_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_1: write-var except <= s_12_0
        fn_state.except = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // D s_13_1: read-var wfxtype:u32
        let s_13_1: u32 = fn_state.wfxtype;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var except:struct
        let s_14_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_1: write-var except <= s_14_0
        fn_state.except = s_14_0;
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // S s_14_3: call Bit(s_14_2)
        let s_14_3: bool = Bit(state, tracer, s_14_2);
        // D s_14_4: read-var except:struct
        let s_14_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_5: write-var except <= s_14_4
        fn_state.except = s_14_4;
        // C s_14_6: const #() : ()
        let s_14_6: () = ();
        // S s_14_7: call ThisInstr(s_14_6)
        let s_14_7: u32 = ThisInstr(state, tracer, s_14_6);
        // D s_14_8: read-var except:struct
        let s_14_8: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_9: write-var except <= s_14_8
        fn_state.except = s_14_8;
        // N s_14_10: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // D s_15_1: read-var wfxtype:u32
        let s_15_1: u32 = fn_state.wfxtype;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var except:struct
        let s_16_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_16_1: write-var except <= s_16_0
        fn_state.except = s_16_0;
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // S s_16_3: call Bit(s_16_2)
        let s_16_3: bool = Bit(state, tracer, s_16_2);
        // D s_16_4: read-var except:struct
        let s_16_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_16_5: write-var except <= s_16_4
        fn_state.except = s_16_4;
        // C s_16_6: const #() : ()
        let s_16_6: () = ();
        // S s_16_7: call ThisInstr(s_16_6)
        let s_16_7: u32 = ThisInstr(state, tracer, s_16_6);
        // D s_16_8: read-var except:struct
        let s_16_8: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_16_9: write-var except <= s_16_8
        fn_state.except = s_16_8;
        // N s_16_10: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
