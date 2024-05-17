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
use SoftwareStep_DidNotStep::*;
use Bit::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use SoftwareStep_SteppedEX::*;
use common::*;
pub fn AArch64_SoftwareStepException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25291: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_25294: bool,
        gs_25295: bool,
        except: ProductTypeb7f99f96751e17c4,
        ga_19816: bool,
        preferred_exception_return: u64,
        vect_offset: i64,
        gs_25296: bool,
        gs_25293: bool,
        gs_25311: bool,
        gs_25291: (),
    }
    let fn_state = FunctionState {
        gs_25291,
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
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-ne s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) != (s_0_5));
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #16975u : u32
        let s_0_8: u32 = 16975;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // C s_0_11: const #448u : u32
        let s_0_11: u32 = 448;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: u8 = {
            let value = state.read_register::<u8>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_10 s_0_13
        let s_0_14: bool = ((s_0_10) == (s_0_13));
        // N s_0_15: branch s_0_14 b23 b1
        if s_0_14 {
            return block_23(state, tracer, fn_state);
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
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
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
        // D s_1_7: write-var gs#25293 <= s_1_6
        fn_state.gs_25293 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25293:u8
        let s_2_0: bool = fn_state.gs_25293;
        // N s_2_1: branch s_2_0 b22 b3
        if s_2_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#25294 <= s_3_0
        fn_state.gs_25294 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25294:u8
        let s_4_0: bool = fn_state.gs_25294;
        // N s_4_1: branch s_4_0 b18 b5
        if s_4_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#25296 <= s_5_0
        fn_state.gs_25296 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25296:u8
        let s_6_0: bool = fn_state.gs_25296;
        // D s_6_1: write-var route_to_el2 <= s_6_0
        fn_state.route_to_el2 = s_6_0;
        // C s_6_2: const #64s : i64
        let s_6_2: i64 = 64;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // S s_6_4: call ThisInstrAddr(s_6_3)
        let s_6_4: Bits = ThisInstrAddr(state, tracer, s_6_3);
        // S s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // D s_6_6: write-var preferred_exception_return <= s_6_5
        fn_state.preferred_exception_return = s_6_5;
        // C s_6_7: const #0u : u8
        let s_6_7: u8 = 0;
        // C s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 4u16);
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (s_6_8.value() as i128);
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: write-var vect_offset <= s_6_10
        fn_state.vect_offset = s_6_10;
        // C s_6_12: const #26u : u32
        let s_6_12: u32 = 26;
        // S s_6_13: call ExceptionSyndrome(s_6_12)
        let s_6_13: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_6_12,
        );
        // D s_6_14: write-var except <= s_6_13
        fn_state.except = s_6_13;
        // C s_6_15: const #() : ()
        let s_6_15: () = ();
        // S s_6_16: call SoftwareStep_DidNotStep(s_6_15)
        let s_6_16: bool = SoftwareStep_DidNotStep(state, tracer, s_6_15);
        // N s_6_17: branch s_6_16 b17 b7
        if s_6_16 {
            return block_17(state, tracer, fn_state);
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
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call SoftwareStep_SteppedEX(s_7_4)
        let s_7_5: bool = SoftwareStep_SteppedEX(state, tracer, s_7_4);
        // N s_7_6: branch s_7_5 b16 b8
        if s_7_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var ga#19816 <= s_8_0
        fn_state.ga_19816 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#19816:u8
        let s_9_0: bool = fn_state.ga_19816;
        // D s_9_1: call Bit(s_9_0)
        let s_9_1: bool = Bit(state, tracer, s_9_0);
        // D s_9_2: read-var except:struct
        let s_9_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_9_3: write-var except <= s_9_2
        fn_state.except = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var except:struct
        let s_10_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_10_1: write-var except <= s_10_0
        fn_state.except = s_10_0;
        // C s_10_2: const #16975u : u32
        let s_10_2: u32 = 16975;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 2u16);
        // C s_10_5: const #432u : u32
        let s_10_5: u32 = 432;
        // D s_10_6: read-reg s_10_5:u8
        let s_10_6: u8 = {
            let value = state.read_register::<u8>(s_10_5 as isize);
            tracer.read_register(s_10_5 as isize, value);
            value
        };
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 2u16);
        // D s_10_8: cmp-eq s_10_4 s_10_7
        let s_10_8: bool = ((s_10_4) == (s_10_7));
        // N s_10_9: branch s_10_8 b15 b11
        if s_10_8 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var route_to_el2:u8
        let s_11_0: bool = fn_state.route_to_el2;
        // D s_11_1: write-var gs#25311 <= s_11_0
        fn_state.gs_25311 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#25311:u8
        let s_12_0: bool = fn_state.gs_25311;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
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
        // D s_13_0: read-var vect_offset:i64
        let s_13_0: i64 = fn_state.vect_offset;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // C s_13_2: const #440u : u32
        let s_13_2: u32 = 440;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: read-var except:struct
        let s_13_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_13_5: read-var preferred_exception_return:u64
        let s_13_5: u64 = fn_state.preferred_exception_return;
        // D s_13_6: call AArch64_TakeException(s_13_3, s_13_4, s_13_5, s_13_1)
        let s_13_6: () = AArch64_TakeException(
            state,
            tracer,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_1,
        );
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var vect_offset:i64
        let s_14_0: i64 = fn_state.vect_offset;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // C s_14_2: const #432u : u32
        let s_14_2: u32 = 432;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: u8 = {
            let value = state.read_register::<u8>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: read-var except:struct
        let s_14_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_5: read-var preferred_exception_return:u64
        let s_14_5: u64 = fn_state.preferred_exception_return;
        // D s_14_6: call AArch64_TakeException(s_14_3, s_14_4, s_14_5, s_14_1)
        let s_14_6: () = AArch64_TakeException(
            state,
            tracer,
            s_14_3,
            s_14_4,
            s_14_5,
            s_14_1,
        );
        // N s_14_7: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#25311 <= s_15_0
        fn_state.gs_25311 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var ga#19816 <= s_16_0
        fn_state.ga_19816 = s_16_0;
        // N s_16_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // S s_17_1: call Bit(s_17_0)
        let s_17_1: bool = Bit(state, tracer, s_17_0);
        // D s_17_2: read-var except:struct
        let s_17_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_17_3: write-var except <= s_17_2
        fn_state.except = s_17_2;
        // N s_17_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #102552u : u32
        let s_18_0: u32 = 102552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_HCR_EL2_Type_TGE(s_18_1)
        let s_18_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b21 b19
        if s_18_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #104880u : u32
        let s_19_0: u32 = 104880;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_MDCR_EL2_Type_TDE(s_19_1)
        let s_19_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#25295 <= s_19_6
        fn_state.gs_25295 = s_19_6;
        // N s_19_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#25295:u8
        let s_20_0: bool = fn_state.gs_25295;
        // D s_20_1: write-var gs#25296 <= s_20_0
        fn_state.gs_25296 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#25295 <= s_21_0
        fn_state.gs_25295 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // D s_22_2: write-var gs#25294 <= s_22_1
        fn_state.gs_25294 = s_22_1;
        // N s_22_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#25293 <= s_23_0
        fn_state.gs_25293 = s_23_0;
        // N s_23_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
