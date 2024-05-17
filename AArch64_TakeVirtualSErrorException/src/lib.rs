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
use u_get_VSESR_EL2_Type_IDS::*;
use EL2Enabled::*;
use u_get_VSESR_EL2_Type_ISS::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use u__IMPDEF_bits::*;
use Bit::*;
use u_get_HCR_EL2_Type_TGE::*;
use HaveRASExt::*;
use u_get_HCR_EL2_Type_AMO::*;
use ClearPendingVirtualSError::*;
use common::*;
pub fn AArch64_TakeVirtualSErrorException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25215: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        except: ProductTypeb7f99f96751e17c4,
        gs_25218: bool,
        preferred_exception_return: u64,
        gs_25216: bool,
        vect_offset: i64,
        gs_25217: bool,
        syndrome: u32,
        gs_25215: (),
    }
    let fn_state = FunctionState {
        gs_25215,
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
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
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
        // N s_0_7: branch s_0_6 b15 b1
        if s_0_6 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#25216 <= s_1_6
        fn_state.gs_25216 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25216:u8
        let s_2_0: bool = fn_state.gs_25216;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#25217 <= s_3_0
        fn_state.gs_25217 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25217:u8
        let s_4_0: bool = fn_state.gs_25217;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #102552u : u32
        let s_4_2: u32 = 102552;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: call _get_HCR_EL2_Type_TGE(s_4_3)
        let s_4_4: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_4_3);
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // C s_4_6: const #0u : u8
        let s_4_6: bool = false;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // D s_4_8: cmp-eq s_4_5 s_4_7
        let s_4_8: bool = ((s_4_5) == (s_4_7));
        // N s_4_9: branch s_4_8 b13 b5
        if s_4_8 {
            return block_13(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#25218 <= s_5_0
        fn_state.gs_25218 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25218:u8
        let s_6_0: bool = fn_state.gs_25218;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
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
        // C s_6_7: const #384u : u12
        let s_6_7: u16 = 384;
        // C s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 12u16);
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (s_6_8.value() as i128);
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: write-var vect_offset <= s_6_10
        fn_state.vect_offset = s_6_10;
        // C s_6_12: const #24u : u32
        let s_6_12: u32 = 24;
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
        // S s_6_16: call HaveRASExt(s_6_15)
        let s_6_16: bool = HaveRASExt(state, tracer, s_6_15);
        // N s_6_17: branch s_6_16 b12 b7
        if s_6_16 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #25s : i64
        let s_7_0: i64 = 25;
        // C s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // C s_7_2: const #"Virtual SError syndrome" : str
        let s_7_2: &'static str = "Virtual SError syndrome";
        // S s_7_3: call __IMPDEF_bits(s_7_1, s_7_2)
        let s_7_3: Bits = u__IMPDEF_bits(state, tracer, s_7_1, s_7_2);
        // S s_7_4: cast reint s_7_3 -> u25
        let s_7_4: u32 = (s_7_3.value() as u32);
        // D s_7_5: write-var syndrome <= s_7_4
        fn_state.syndrome = s_7_4;
        // C s_7_6: const #24s : i
        let s_7_6: i128 = 24;
        // D s_7_7: read-var syndrome:u25
        let s_7_7: u32 = fn_state.syndrome;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 25u16);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: bit-extract s_7_8 s_7_6 s_7_9
        let s_7_10: Bits = (Bits::new(
            ((s_7_8) >> (s_7_6)).value(),
            u16::try_from(s_7_9).unwrap(),
        ));
        // D s_7_11: cast reint s_7_10 -> u8
        let s_7_11: bool = ((s_7_10.value()) != 0);
        // C s_7_12: const #0s : i
        let s_7_12: i128 = 0;
        // C s_7_13: const #0u : u64
        let s_7_13: u64 = 0;
        // D s_7_14: cast zx s_7_11 -> u64
        let s_7_14: u64 = (s_7_11 as u64);
        // C s_7_15: const #1u : u64
        let s_7_15: u64 = 1;
        // D s_7_16: and s_7_14 s_7_15
        let s_7_16: u64 = ((s_7_14) & (s_7_15));
        // D s_7_17: cmp-eq s_7_16 s_7_15
        let s_7_17: bool = ((s_7_16) == (s_7_15));
        // D s_7_18: lsl s_7_14 s_7_12
        let s_7_18: u64 = s_7_14 << s_7_12;
        // D s_7_19: or s_7_13 s_7_18
        let s_7_19: u64 = ((s_7_13) | (s_7_18));
        // D s_7_20: cmpl s_7_18
        let s_7_20: u64 = !s_7_18;
        // D s_7_21: and s_7_13 s_7_20
        let s_7_21: u64 = ((s_7_13) & (s_7_20));
        // D s_7_22: select s_7_17 s_7_19 s_7_21
        let s_7_22: u64 = if s_7_17 { s_7_19 } else { s_7_21 };
        // D s_7_23: cast trunc s_7_22 -> u8
        let s_7_23: bool = ((s_7_22) != 0);
        // D s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 1u16);
        // C s_7_25: const #1u : u8
        let s_7_25: bool = true;
        // C s_7_26: cast zx s_7_25 -> bv
        let s_7_26: Bits = Bits::new(s_7_25 as u128, 1u16);
        // D s_7_27: cmp-eq s_7_24 s_7_26
        let s_7_27: bool = ((s_7_24) == (s_7_26));
        // N s_7_28: branch s_7_27 b11 b8
        if s_7_27 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call ClearPendingVirtualSError(s_10_0)
        let s_10_1: () = ClearPendingVirtualSError(state, tracer, s_10_0);
        // D s_10_2: read-var vect_offset:i64
        let s_10_2: i64 = fn_state.vect_offset;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // C s_10_4: const #440u : u32
        let s_10_4: u32 = 440;
        // D s_10_5: read-reg s_10_4:u8
        let s_10_5: u8 = {
            let value = state.read_register::<u8>(s_10_4 as isize);
            tracer.read_register(s_10_4 as isize, value);
            value
        };
        // D s_10_6: read-var except:struct
        let s_10_6: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_10_7: read-var preferred_exception_return:u64
        let s_10_7: u64 = fn_state.preferred_exception_return;
        // D s_10_8: call AArch64_TakeException(s_10_5, s_10_6, s_10_7, s_10_3)
        let s_10_8: () = AArch64_TakeException(
            state,
            tracer,
            s_10_5,
            s_10_6,
            s_10_7,
            s_10_3,
        );
        // N s_10_9: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var syndrome:u25
        let s_11_0: u32 = fn_state.syndrome;
        // D s_11_1: write-var except.6 <= s_11_0
        fn_state.except._6 = s_11_0;
        // N s_11_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #100824u : u32
        let s_12_0: u32 = 100824;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_VSESR_EL2_Type_IDS(s_12_1)
        let s_12_2: bool = u_get_VSESR_EL2_Type_IDS(state, tracer, s_12_1);
        // D s_12_3: call Bit(s_12_2)
        let s_12_3: bool = Bit(state, tracer, s_12_2);
        // D s_12_4: read-var except:struct
        let s_12_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_5: write-var except <= s_12_4
        fn_state.except = s_12_4;
        // C s_12_6: const #100824u : u32
        let s_12_6: u32 = 100824;
        // D s_12_7: read-reg s_12_6:struct
        let s_12_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_6 as isize);
            tracer.read_register(s_12_6 as isize, value);
            value
        };
        // D s_12_8: call _get_VSESR_EL2_Type_ISS(s_12_7)
        let s_12_8: u32 = u_get_VSESR_EL2_Type_ISS(state, tracer, s_12_7);
        // D s_12_9: read-var except:struct
        let s_12_9: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_12_10: write-var except <= s_12_9
        fn_state.except = s_12_9;
        // N s_12_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #102552u : u32
        let s_13_0: u32 = 102552;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call _get_HCR_EL2_Type_AMO(s_13_1)
        let s_13_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_13_1);
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #1u : u8
        let s_13_4: bool = true;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var gs#25218 <= s_13_6
        fn_state.gs_25218 = s_13_6;
        // N s_13_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // D s_14_2: write-var gs#25217 <= s_14_1
        fn_state.gs_25217 = s_14_1;
        // N s_14_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#25216 <= s_15_0
        fn_state.gs_25216 = s_15_0;
        // N s_15_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
